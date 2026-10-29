use crate::diagnostics::severity::DiagnosticSeverity;
use crate::diagnostics::span::SourceSpan;
use crate::diagnostics::types::*;
use crate::frontend::ast::nodes::*;
use crate::frontend::lexer::scanner::lex;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
    pub indent_stack: Vec<u32>,
    pub errors: Vec<ParserError>,
    pub warnings: Vec<WarningDiagnostic>,
    pub recover_errors: bool,
}

impl Parser {
    pub fn report_error(&mut self, error: ParserError) {
        self.errors.push(error);
    }

    pub fn synchronize(&mut self) {
        while !self.at(Some(TokenKind::Eof), None) {
            if self.at(Some(TokenKind::Newline), None) {
                let _ = self.eat(Some(TokenKind::Newline), None);

                return;
            }

            self.pos += 1;
        }
    }

    // Constructor

    pub fn new(src: &str, recover_errors: bool) -> Result<Self, String> {
        let tokens = lex(src)?;

        Ok(Self {
            tokens,
            pos: 0,
            indent_stack: vec![0],
            errors: Vec::new(),
            warnings: Vec::new(),
            recover_errors,
        })
    }

    // Navigation helpers

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn current_column(&self) -> Option<u32> {
        self.current().map(|token| token.column)
    }

    pub fn previous(&self) -> Option<&Token> {
        self.pos
            .checked_sub(1)
            .and_then(|position| self.tokens.get(position))
    }

    pub fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.pos + offset)
    }

    pub fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    pub fn at_end(&self) -> bool {
        match self.current() {
            Some(token) => token.kind == TokenKind::Eof,
            None => true,
        }
    }

    // Token checking helpers

    pub fn at(&self, kind: Option<TokenKind>, lexeme: Option<&str>) -> bool {
        let Some(token) = self.current() else {
            return false;
        };

        if let Some(expected_kind) = kind {
            if token.kind != expected_kind {
                return false;
            }
        }

        if let Some(expected_lexeme) = lexeme {
            if token.lexeme != expected_lexeme {
                return false;
            }
        }

        true
    }

    pub fn next_is(&self, kind: TokenKind) -> bool {
        match self.peek(1) {
            Some(token) => token.kind == kind,
            None => false,
        }
    }

    pub fn eat(
        &mut self,
        kind: Option<TokenKind>,
        val: Option<&str>,
    ) -> Result<Token, ParserError> {
        let token = self.current().cloned().unwrap();

        if let Some(expected_kind) = kind {
            if token.kind != expected_kind {
                return Err(ParserError::new(
                    "NXD-P1001",
                    format!(
                        "NXD-P1001: Expected {:?}, got {:?}",
                        expected_kind, token.kind,
                    ),
                    token.span(),
                ));
            }
        }

        if let Some(expected_val) = val {
            if token.lexeme != expected_val {
                return Err(ParserError::new(
                    "NXD-P1002",
                    format!("NXD-P1002: Expected {}, got {}", expected_val, token.lexeme,),
                    token.span(),
                ));
            }
        }

        self.advance();

        Ok(token)
    }
    // Parser entry points

    pub fn parse_program(&mut self) -> Result<AstModule, ParserError> {
        while self.at(Some(TokenKind::Newline), None) {
            self.eat(Some(TokenKind::Newline), None)?;
        }
        if self.at(Some(TokenKind::Keyword), Some("MODULE")) {
            return self.parse_module();
        }

        let mut imports: Vec<AstImport> = Vec::new();
        let mut body: Vec<AstNode> = Vec::new();

        while !self.at(Some(TokenKind::Eof), None) {
            if self.at(Some(TokenKind::Newline), None) {
                self.eat(Some(TokenKind::Newline), None)?;

                continue;
            }

            if self.at(Some(TokenKind::Keyword), Some("IMPORT")) {
                imports.push(self.parse_import()?);
                continue;
            }

            let node = self.parse_top_level()?;
            body.push(node);
        }

        Ok(AstModule {
            name: "ANONYMOUS".to_string(),
            imports,
            body,
        })
    }

    // Top level grammer

    pub fn parse_module(&mut self) -> Result<AstModule, ParserError> {
        self.eat(Some(TokenKind::Keyword), Some("MODULE"))?;

        let name_token = self.eat(Some(TokenKind::Ident), None)?;

        let name = name_token.lexeme;

        let mut imports = Vec::new();
        let mut body = Vec::new();

        while !self.at_end() {
            if self.at(Some(TokenKind::Newline), None) {
                self.eat(Some(TokenKind::Newline), None)?;

                continue;
            }

            if self.at(Some(TokenKind::Keyword), Some("IMPORT")) {
                match self.parse_import() {
                    Ok(import) => {
                        imports.push(import);
                    }

                    Err(error) => {
                        if !self.recover_errors {
                            return Err(error);
                        }

                        self.report_error(error.clone());

                        eprintln!(
                            "Collected: {} {} {}:{}",
                            error.code, error.message, error.span.line, error.span.column,
                        );

                        self.synchronize();
                    }
                }

                continue;
            }
            match self.parse_top_level() {
                Ok(node) => {
                    body.push(node);
                }

                Err(error) => {
                    if !self.recover_errors {
                        return Err(error);
                    }

                    self.report_error(error.clone());

                    eprintln!(
                        "Collected: {} {} {}:{}",
                        error.code, error.message, error.span.line, error.span.column,
                    );

                    self.synchronize();
                }
            }
        }
        Ok(AstModule {
            name,
            imports,
            body,
        })
    }

    pub fn parse_import(&mut self) -> Result<AstImport, ParserError> {
        let import_token = self.eat(Some(TokenKind::Keyword), Some("IMPORT"))?;

        let path_token = self.eat(Some(TokenKind::Ident), None)?;

        let mut alias = None;
        let mut end_span = path_token.span();

        if self.at(Some(TokenKind::Op), Some("AS")) {
            self.eat(Some(TokenKind::Op), Some("AS"))?;

            let alias_token = self.eat(Some(TokenKind::Ident), None)?;

            end_span = alias_token.span();
            alias = Some(alias_token.lexeme);
        }

        Ok(AstImport {
            path: path_token.lexeme,
            alias,
            span: SourceSpan {
                line: import_token.line,
                column: import_token.column,
                end_line: end_span.end_line,
                end_column: end_span.end_column,
            },
        })
    }

    pub fn parse_top_level(&mut self) -> Result<AstNode, ParserError> {
        if self.at(Some(TokenKind::Keyword), Some("TYPE")) {
            return self.parse_type_decl();
        }

        if self.at(Some(TokenKind::Keyword), Some("TRAIT")) {
            return self.parse_trait_decl();
        }

        if self.at(Some(TokenKind::Keyword), Some("IMPL")) {
            return self.parse_impl_decl();
        }

        if self.at(Some(TokenKind::Keyword), Some("ASYNC")) {
            self.eat(Some(TokenKind::Keyword), Some("ASYNC"))?;

            let is_async = true;

            return self.parse_func_decl(is_async);
        }

        if self.at(Some(TokenKind::Keyword), Some("FUNC")) {
            let is_async = false;

            return self.parse_func_decl(is_async);
        }

        let statement = self.parse_statement()?;

        Ok(AstNode::Statement(statement))
    }

    // Declarations

    pub fn parse_type_decl(&mut self) -> Result<AstNode, ParserError> {
        self.eat(Some(TokenKind::Keyword), Some("TYPE"))?;

        let name_token = self.eat(Some(TokenKind::Ident), None)?;

        let name = name_token.lexeme;

        if self.at(Some(TokenKind::Keyword), Some("ENUM")) {
            self.eat(Some(TokenKind::Keyword), Some("ENUM"))?;

            return self.parse_enum_type(name);
        }

        if self.at(Some(TokenKind::Keyword), Some("UNION")) {
            self.eat(Some(TokenKind::Keyword), Some("UNION"))?;

            return self.parse_union_type(name);
        }

        if self.at(Some(TokenKind::Keyword), Some("TRAIT")) {
            self.eat(Some(TokenKind::Keyword), Some("TRAIT"))?;

            return self.parse_trait_type(name);
        }

        if self.at(Some(TokenKind::Keyword), Some("STRUCT")) {
            self.eat(Some(TokenKind::Keyword), Some("STRUCT"))?;

            return self.parse_struct_type(name);
        }

        todo!("NXD-P1003");
    }

    pub fn parse_trait_decl(&mut self) -> Result<AstNode, ParserError> {
        self.eat(Some(TokenKind::Keyword), Some("TRAIT"))?;

        let name_token = self.eat(Some(TokenKind::Ident), None)?;

        let name = name_token.lexeme;

        self.parse_trait_type(name)
    }

    pub fn parse_impl_decl(&mut self) -> Result<AstNode, ParserError> {
        let impl_column = self.current().map(|token| token.column).unwrap_or(0);

        self.eat(Some(TokenKind::Keyword), Some("IMPL"))?;

        let trait_name_token = self.eat(Some(TokenKind::Ident), None)?;

        let trait_name = trait_name_token.lexeme;

        self.eat(Some(TokenKind::Keyword), Some("FOR"))?;

        let target_type_token = self.eat(Some(TokenKind::Ident), None)?;

        let target_type = target_type_token.lexeme;

        self.eat(Some(TokenKind::Colon), None)?;

        while self.at(Some(TokenKind::Newline), None) {
            self.eat(Some(TokenKind::Newline), None)?;
        }

        let mut methods: Vec<AstFunction> = Vec::new();

        while self.at(Some(TokenKind::Keyword), Some("FUNC"))
            && self
                .current_column()
                .is_some_and(|column| column > impl_column)
        {
            let method_column = self.current_column().unwrap_or(impl_column);

            let func_token = self.eat(Some(TokenKind::Keyword), Some("FUNC"))?;

            let method_name_token = self.eat(Some(TokenKind::Ident), None)?;

            let method_name = method_name_token.lexeme;

            self.eat(Some(TokenKind::LParen), None)?;

            let mut params: Vec<AstParam> = Vec::new();

            if !self.at(Some(TokenKind::RParen), None) {
                let parameter_token = self.eat(Some(TokenKind::Ident), None)?;

                params.push(AstParam {
                    name: parameter_token.lexeme,
                    ty: "any".to_string(),
                });

                while self.at(Some(TokenKind::Comma), None) {
                    self.eat(Some(TokenKind::Comma), None)?;

                    let parameter_token = self.eat(Some(TokenKind::Ident), None)?;

                    params.push(AstParam {
                        name: parameter_token.lexeme,
                        ty: "any".to_string(),
                    });
                }
            }

            let rparen_token = self.eat(Some(TokenKind::RParen), None)?;

            let mut return_type: Option<String> = None;
            let mut method_end_span = rparen_token.span();

            if self.at(Some(TokenKind::Colon), None) {
                self.eat(Some(TokenKind::Colon), None)?;

                if !self.at(Some(TokenKind::Newline), None) {
                    let return_type_token = if self.at(Some(TokenKind::LowType), None) {
                        self.eat(Some(TokenKind::LowType), None)?
                    } else {
                        self.eat(Some(TokenKind::Ident), None)?
                    };

                    method_end_span = return_type_token.span();
                    return_type = Some(return_type_token.lexeme);
                }
            }

            if self.at(Some(TokenKind::Newline), None) {
                self.eat(Some(TokenKind::Newline), None)?;
            }

            let mut body: Vec<AstStatement> = Vec::new();

            while !self.at(Some(TokenKind::Eof), None) {
                while self.at(Some(TokenKind::Newline), None) {
                    self.eat(Some(TokenKind::Newline), None)?;
                }

                if self.at(Some(TokenKind::Eof), None) {
                    break;
                }

                match self.current_column() {
                    Some(column) if column <= method_column => break,
                    None => break,
                    _ => {}
                }

                let statement = self.parse_statement()?;
                body.push(statement);

                if let Some(previous_token) = self.previous() {
                    method_end_span = previous_token.span();
                }
            }

            methods.push(AstFunction {
                name: method_name,
                span: SourceSpan {
                    line: func_token.line,
                    column: func_token.column,
                    end_line: method_end_span.end_line,
                    end_column: method_end_span.end_column,
                },
                params,
                return_type,
                body,
                is_async: false,
            });
        }

        Ok(AstNode::Impl(AstImpl {
            trait_name,
            target_type,
            methods,
        }))
    }

    pub fn parse_func_decl(&mut self, is_async: bool) -> Result<AstNode, ParserError> {
        let func_token = self.eat(Some(TokenKind::Keyword), Some("FUNC"))?;

        let name_token = self.eat(Some(TokenKind::Ident), None)?;

        let name = name_token.lexeme;

        self.eat(Some(TokenKind::LParen), None)?;

        let mut parsed_params: Vec<(String, String)> = Vec::new();

        if !self.at(Some(TokenKind::RParen), None) {
            parsed_params.push(self.parse_param()?);

            while self.at(Some(TokenKind::Comma), None) {
                self.eat(Some(TokenKind::Comma), None)?;

                parsed_params.push(self.parse_param()?);
            }
        }

        let rparen_token = self.eat(Some(TokenKind::RParen), None)?;

        let mut return_type: Option<String> = None;
        let mut function_end_span = rparen_token.span();

        if self.at(Some(TokenKind::Colon), None) {
            self.eat(Some(TokenKind::Colon), None)?;

            // This colon may introduce a return type or begin the block.
            if !self.at(Some(TokenKind::Newline), None) {
                let return_type_token = if self.at(Some(TokenKind::LowType), None) {
                    self.eat(Some(TokenKind::LowType), None)?
                } else {
                    self.eat(Some(TokenKind::Ident), None)?
                };

                function_end_span = return_type_token.span();
                return_type = Some(return_type_token.lexeme);
            }
        }

        self.expect_block_colon()?;

        let body = self.parse_block()?;

        if let Some(previous_token) = self.previous() {
            function_end_span = previous_token.span();
        }

        let params: Vec<AstParam> = parsed_params
            .into_iter()
            .map(|(parameter_name, parameter_type)| AstParam {
                name: parameter_name,
                ty: parameter_type,
            })
            .collect();

        Ok(AstNode::Function(AstFunction {
            name,
            span: SourceSpan {
                line: func_token.line,
                column: func_token.column,
                end_line: function_end_span.end_line,
                end_column: function_end_span.end_column,
            },
            params,
            return_type,
            body,
            is_async,
        }))
    }

    pub fn parse_param(&mut self) -> Result<(String, String), ParserError> {
        let name_token = self.eat(Some(TokenKind::Ident), None)?;

        let name = name_token.lexeme;

        if self.at(Some(TokenKind::Colon), None) {
            self.eat(Some(TokenKind::Colon), None)?;

            let ty = self.parse_type_ref()?;

            return Ok((name, ty));
        }

        Ok((name, "any".to_string()))
    }

    pub fn expect_block_colon(&mut self) -> Result<(), ParserError> {
        if self.at(Some(TokenKind::Colon), None) {
            self.eat(Some(TokenKind::Colon), None)?;
        }

        if self.at(Some(TokenKind::Newline), None) {
            self.eat(Some(TokenKind::Newline), None)?;
        }

        Ok(())
    }

    pub fn parse_block(&mut self) -> Result<Vec<AstStatement>, ParserError> {
        let mut statements: Vec<AstStatement> = Vec::new();

        // Ignore blank lines before the block's first statement.
        while self.at(Some(TokenKind::Newline), None) {
            self.eat(Some(TokenKind::Newline), None)?;
        }

        if self.at(Some(TokenKind::Eof), None) {
            return Ok(statements);
        }

        // The first statement defines this block's indentation.
        let block_column = self.current_column().unwrap_or(0);

        self.indent_stack.push(block_column);

        // Use a closure so indent_stack is popped whether parsing
        // succeeds or returns a ParserError.
        let result = (|| -> Result<(), ParserError> {
            while !self.at(Some(TokenKind::Eof), None) {
                if self.at(Some(TokenKind::Newline), None) {
                    self.eat(Some(TokenKind::Newline), None)?;

                    continue;
                }

                let current_column = self.current_column().unwrap_or(0);

                // Dedent closes the current block.
                if current_column < block_column {
                    break;
                }

                // The enclosing parser handles these tokens.
                if self.at(Some(TokenKind::Keyword), Some("ELSE"))
                    || self.at(Some(TokenKind::Keyword), Some("CASE"))
                    || self.at(Some(TokenKind::Keyword), Some("OTHERWISE"))
                    || self.at(Some(TokenKind::Keyword), Some("CATCH"))
                    || self.at(Some(TokenKind::Keyword), Some("FINALLY"))
                {
                    break;
                }

                let statement = self.parse_statement()?;
                statements.push(statement);
            }

            Ok(())
        })();

        self.indent_stack.pop();

        result?;

        Ok(statements)
    }

    pub fn parse_enum_type(&mut self, name: String) -> Result<AstNode, ParserError> {
        self.eat(Some(TokenKind::Colon), None)?;

        while self.at(Some(TokenKind::Newline), None) {
            self.eat(Some(TokenKind::Newline), None)?;
        }

        let mut variants: Vec<String> = Vec::new();

        while self.at(Some(TokenKind::Ident), None) {
            let variant = self.eat(Some(TokenKind::Ident), None)?;

            variants.push(variant.lexeme);

            if self.at(Some(TokenKind::Comma), None) {
                self.eat(Some(TokenKind::Comma), None)?;
            }

            while self.at(Some(TokenKind::Newline), None) {
                self.eat(Some(TokenKind::Newline), None)?;
            }
        }

        Ok(AstNode::Enum(AstEnum { name, variants }))
    }

    pub fn parse_union_type(&mut self, name: String) -> Result<AstNode, ParserError> {
        self.eat(Some(TokenKind::Colon), None)?;

        while self.at(Some(TokenKind::Newline), None) {
            self.eat(Some(TokenKind::Newline), None)?;
        }

        let mut variants: Vec<AstUnionVariant> = Vec::new();

        while self.at(Some(TokenKind::Ident), None) {
            let kind_token = self.eat(Some(TokenKind::Ident), None)?;

            let kind = kind_token.lexeme;

            self.eat(Some(TokenKind::LBrace), None)?;

            let mut fields: Vec<AstField> = Vec::new();

            while self.at(Some(TokenKind::Newline), None) {
                self.eat(Some(TokenKind::Newline), None)?;
            }

            while !self.at(Some(TokenKind::RBrace), None) {
                if self.at(Some(TokenKind::Newline), None) {
                    self.eat(Some(TokenKind::Newline), None)?;

                    continue;
                }

                let field_name_token = self.eat(Some(TokenKind::Ident), None)?;

                self.eat(Some(TokenKind::Colon), None)?;

                let field_type = self.parse_type_ref()?;

                fields.push(AstField {
                    name: field_name_token.lexeme,
                    ty: field_type,
                });

                if self.at(Some(TokenKind::Comma), None) {
                    self.eat(Some(TokenKind::Comma), None)?;
                }
            }

            self.eat(Some(TokenKind::RBrace), None)?;

            variants.push(AstUnionVariant { kind, fields });

            while self.at(Some(TokenKind::Newline), None) {
                self.eat(Some(TokenKind::Newline), None)?;
            }
        }

        Ok(AstNode::Union(AstUnion { name, variants }))
    }

    pub fn parse_trait_type(&mut self, name: String) -> Result<AstNode, ParserError> {
        self.eat(Some(TokenKind::Colon), None)?;

        while self.at(Some(TokenKind::Newline), None) {
            self.eat(Some(TokenKind::Newline), None)?;
        }

        let mut methods: Vec<AstFunctionSignature> = Vec::new();

        while self.at(Some(TokenKind::Keyword), Some("FUNC")) {
            let func_token = self.eat(Some(TokenKind::Keyword), Some("FUNC"))?;

            let method_name_token = self.eat(Some(TokenKind::Ident), None)?;

            let method_name = method_name_token.lexeme;

            self.eat(Some(TokenKind::LParen), None)?;

            let mut params: Vec<AstParam> = Vec::new();

            if !self.at(Some(TokenKind::RParen), None) {
                let parameter_token = self.eat(Some(TokenKind::Ident), None)?;

                params.push(AstParam {
                    name: parameter_token.lexeme,
                    ty: "any".to_string(),
                });

                while self.at(Some(TokenKind::Comma), None) {
                    self.eat(Some(TokenKind::Comma), None)?;

                    let parameter_token = self.eat(Some(TokenKind::Ident), None)?;

                    params.push(AstParam {
                        name: parameter_token.lexeme,
                        ty: "any".to_string(),
                    });
                }
            }

            let rparen_token = self.eat(Some(TokenKind::RParen), None)?;

            let mut return_type: Option<String> = None;
            let mut end_span = rparen_token.span();

            if self.at(Some(TokenKind::Colon), None) {
                self.eat(Some(TokenKind::Colon), None)?;

                let return_type_token = if self.at(Some(TokenKind::LowType), None) {
                    self.eat(Some(TokenKind::LowType), None)?
                } else {
                    self.eat(Some(TokenKind::Ident), None)?
                };

                end_span = return_type_token.span();
                return_type = Some(return_type_token.lexeme);
            }

            methods.push(AstFunctionSignature {
                name: method_name,
                span: SourceSpan {
                    line: func_token.line,
                    column: func_token.column,
                    end_line: end_span.end_line,
                    end_column: end_span.end_column,
                },
                params,
                return_type,
            });

            while self.at(Some(TokenKind::Newline), None) {
                self.eat(Some(TokenKind::Newline), None)?;
            }
        }

        Ok(AstNode::Trait(AstTrait { name, methods }))
    }

    pub fn parse_struct_type(&mut self, name: String) -> Result<AstNode, ParserError> {
        self.eat(Some(TokenKind::Colon), None)?;

        while self.at(Some(TokenKind::Newline), None) {
            self.eat(Some(TokenKind::Newline), None)?;
        }

        let mut fields: Vec<AstField> = Vec::new();

        while self.at(Some(TokenKind::Ident), None) && self.next_is(TokenKind::Colon) {
            let field_name_token = self.eat(Some(TokenKind::Ident), None)?;

            self.eat(Some(TokenKind::Colon), None)?;

            let field_type = self.parse_type_ref()?;

            if self.at(Some(TokenKind::Comma), None) {
                self.eat(Some(TokenKind::Comma), None)?;
            }

            fields.push(AstField {
                name: field_name_token.lexeme,
                ty: field_type,
            });

            while self.at(Some(TokenKind::Newline), None) {
                self.eat(Some(TokenKind::Newline), None)?;
            }
        }

        Ok(AstNode::Struct(AstStruct { name, fields }))
    }

    pub fn parse_type_ref(&mut self) -> Result<String, ParserError> {
        if self.at(Some(TokenKind::LowType), None) {
            let token = self.eat(Some(TokenKind::LowType), None)?;

            return Ok(token.lexeme);
        }

        let token = self.eat(Some(TokenKind::Ident), None)?;

        Ok(token.lexeme)
    }

    // Statements

    pub fn parse_statement(&mut self) -> Result<AstStatement, ParserError> {
        let token = self.current().cloned().unwrap();

        if token.kind == TokenKind::Keyword {
            match token.lexeme.as_str() {
                "LET" => {
                    return Ok(AstStatement::Let(self.parse_let()?));
                }

                "CONST" => {
                    return Ok(AstStatement::Const(self.parse_const()?));
                }

                "RETURN" => {
                    return Ok(AstStatement::Return(self.parse_return()?));
                }

                "TRY" => {
                    return Ok(AstStatement::Try(self.parse_try()?));
                }

                "LOOP" => {
                    return Ok(AstStatement::Loop(self.parse_loop()?));
                }

                "IF" => {
                    return Ok(AstStatement::If(self.parse_if()?));
                }

                "MATCH" => {
                    return Ok(AstStatement::Match(self.parse_match()?));
                }

                "SPAWN" => {
                    return Ok(AstStatement::Expr(AstExpr::Call(self.parse_spawn()?)));
                }

                "SEND" => {
                    return Ok(AstStatement::Expr(AstExpr::Call(self.parse_send()?)));
                }

                "RECV" => {
                    return Ok(AstStatement::Expr(AstExpr::Call(self.parse_recv()?)));
                }

                "AWAIT" => {
                    return Ok(AstStatement::Expr(AstExpr::Call(self.parse_await_stmt()?)));
                }

                _ => {}
            }
        }

        if token.kind == TokenKind::Op && token.lexeme == "MOVE" {
            return Ok(AstStatement::Move(self.parse_move()?));
        }

        if token.kind == TokenKind::Op && token.lexeme == "CLONE" {
            return Ok(AstStatement::Clone(self.parse_clone()?));
        }

        let expr = self.parse_expr()?;

        Ok(AstStatement::Expr(expr))
    }

    pub fn parse_clone(&mut self) -> Result<AstClone, ParserError> {
        let token = self.current().cloned().unwrap();

        if token.lexeme != "CLONE" {
            return Err(ParserError::new(
                "NXD-P1004",
                format!(
                    "NXD-P1004: Expected CLONE, got {:?} {}",
                    token.kind, token.lexeme
                ),
                token.span(),
            ));
        }

        let clone_token = self.eat(Some(TokenKind::Op), Some("CLONE"))?;

        if !self.at(Some(TokenKind::Ident), None) {
            let token = self.current().cloned().unwrap();

            return Err(ParserError::new(
                "NXD-P1005",
                format!(
                    "NXD-P1005: Expected source identifier after CLONE, got {:?} {}",
                    token.kind, token.lexeme
                ),
                token.span(),
            ));
        }

        let source_token = self.eat(Some(TokenKind::Ident), None)?;

        let source = AstExpr::Var(AstVar {
            name: source_token.lexeme,
            line: source_token.line,
            column: source_token.column,
        });

        if !(self.at(Some(TokenKind::Ident), None)
            && self.current().map(|token| token.lexeme.as_str()) == Some("TO"))
        {
            let token = self.current().cloned().unwrap();

            return Err(ParserError::new(
                "NXD-P1006",
                format!(
                    "NXD-P1006: Expected TO after CLONE source, got {:?} {}",
                    token.kind, token.lexeme
                ),
                token.span(),
            ));
        }

        self.eat(Some(TokenKind::Ident), None)?;

        if !self.at(Some(TokenKind::Ident), None) {
            let token = self.current().cloned().unwrap();

            return Err(ParserError::new(
                "NXD-P1007",
                format!(
                    "NXD-P1007: Expected target identifier after TO, got {:?} {}",
                    token.kind, token.lexeme
                ),
                token.span(),
            ));
        }

        let target_token = self.eat(Some(TokenKind::Ident), None)?;

        let target = AstExpr::Var(AstVar {
            name: target_token.lexeme,
            line: target_token.line,
            column: target_token.column,
        });

        Ok(AstClone {
            source,
            target,
            line: clone_token.line,
            column: clone_token.column,
        })
    }

    pub fn parse_move(&mut self) -> Result<AstMove, ParserError> {
        let move_token = self.eat(Some(TokenKind::Op), Some("MOVE"))?;

        let source = self.parse_primary()?;

        if !(self.at(Some(TokenKind::Ident), None)
            && self.current().map(|token| token.lexeme.as_str()) == Some("TO"))
        {
            let token = self.current().cloned().unwrap();

            return Err(ParserError::new(
                "NXD-P1011",
                format!(
                    "NXD-P1011: Expected TO after Move source, got \"{}\"",
                    token.lexeme
                ),
                token.span(),
            ));
        }

        self.eat(Some(TokenKind::Ident), None)?;

        let target = self.parse_primary()?;

        Ok(AstMove {
            source,
            target,
            line: move_token.line,
            column: move_token.column,
        })
    }

    pub fn parse_let(&mut self) -> Result<AstLet, ParserError> {
        self.eat(Some(TokenKind::Keyword), Some("LET"))?;

        let ident_token = self.eat(Some(TokenKind::Ident), None)?;

        let name = ident_token.lexeme.clone();

        self.eat(Some(TokenKind::Keyword), Some("SET"))?;

        let value = self.parse_expr()?;

        Ok(AstLet {
            name,
            value,
            line: ident_token.line,
            column: ident_token.column,
        })
    }

    pub fn parse_const(&mut self) -> Result<AstConst, ParserError> {
        self.eat(Some(TokenKind::Keyword), Some("CONST"))?;

        let ident_token = self.eat(Some(TokenKind::Ident), None)?;

        let name = ident_token.lexeme.clone();

        self.eat(Some(TokenKind::Keyword), Some("SET"))?;

        let value = self.parse_expr()?;

        Ok(AstConst {
            name,
            value,
            line: ident_token.line,
            column: ident_token.column,
        })
    }

    pub fn parse_return(&mut self) -> Result<AstReturn, ParserError> {
        let return_token = self.eat(Some(TokenKind::Keyword), Some("RETURN"))?;

        let value = self.parse_expr()?;

        Ok(AstReturn {
            value,
            line: return_token.line,
            column: return_token.column,
        })
    }

    pub fn parse_loop(&mut self) -> Result<AstLoop, ParserError> {
        let loop_token = self.eat(Some(TokenKind::Keyword), Some("LOOP"))?;

        self.expect_block_colon()?;

        let body = self.parse_block()?;

        Ok(AstLoop {
            body,
            line: loop_token.line,
            column: loop_token.column,
        })
    }

    pub fn parse_if(&mut self) -> Result<AstIf, ParserError> {
        let if_token = self.eat(Some(TokenKind::Keyword), Some("IF"))?;

        let condition = self.parse_expr()?;

        self.expect_block_colon()?;

        let then_branch = self.parse_block()?;

        let mut else_branch = Vec::new();

        if self.at(Some(TokenKind::Keyword), Some("ELSE")) {
            self.eat(Some(TokenKind::Keyword), Some("ELSE"))?;

            self.expect_block_colon()?;

            else_branch = self.parse_block()?;
        }

        Ok(AstIf {
            condition,
            then_branch,
            else_branch,
            line: if_token.line,
            column: if_token.column,
        })
    }

    pub fn parse_match(&mut self) -> Result<AstMatch, ParserError> {
        let match_token = self.eat(Some(TokenKind::Keyword), Some("MATCH"))?;

        let scrutinee = self.parse_expr()?;

        self.expect_block_colon()?;

        let mut arms = Vec::new();

        let mut otherwise = None;

        while self.at(Some(TokenKind::Keyword), Some("CASE")) {
            arms.push(self.parse_case_block()?);
        }

        if self.at(Some(TokenKind::Keyword), Some("OTHERWISE")) {
            self.eat(Some(TokenKind::Keyword), Some("OTHERWISE"))?;

            self.expect_block_colon()?;

            otherwise = Some(self.parse_block()?);
        }

        Ok(AstMatch {
            scrutinee,
            arms,
            otherwise,
            line: match_token.line,
            column: match_token.column,
        })
    }

    pub fn parse_try(&mut self) -> Result<AstTry, ParserError> {
        let try_token = self.eat(Some(TokenKind::Keyword), Some("TRY"))?;

        self.eat(Some(TokenKind::Colon), None)?;

        let try_body = self.parse_block()?;

        let mut catch_body = Vec::new();
        let mut finally_body = Vec::new();

        if self.at(Some(TokenKind::Keyword), Some("CATCH")) {
            self.eat(Some(TokenKind::Keyword), Some("CATCH"))?;

            self.eat(Some(TokenKind::Colon), None)?;

            catch_body = self.parse_block()?;
        }

        if self.at(Some(TokenKind::Keyword), Some("FINALLY")) {
            self.eat(Some(TokenKind::Keyword), Some("FINALLY"))?;

            self.eat(Some(TokenKind::Colon), None)?;

            finally_body = self.parse_block()?;
        }

        Ok(AstTry {
            try_body,
            catch_body,
            finally_body,
            line: try_token.line,
            column: try_token.column,
        })
    }

    pub fn parse_case_block(&mut self) -> Result<AstMatchArm, ParserError> {
        self.eat(Some(TokenKind::Keyword), Some("CASE"))?;

        let pattern = self.parse_pattern()?;

        self.expect_block_colon()?;

        let body = self.parse_block()?;

        Ok(AstMatchArm { pattern, body })
    }

    pub fn parse_pattern(&mut self) -> Result<String, ParserError> {
        let token = self.current().cloned().unwrap();

        if matches!(token.kind, TokenKind::Number | TokenKind::String) {
            return Ok(token.lexeme);
        }

        Ok(self.eat(Some(TokenKind::Ident), None)?.lexeme)
    }

    pub fn parse_spawn(&mut self) -> Result<AstCall, ParserError> {
        self.eat(Some(TokenKind::Keyword), Some("SPAWN"))?;

        self.parse_call_expr()
    }

    pub fn parse_send(&mut self) -> Result<AstCall, ParserError> {
        let send_token = self.eat(Some(TokenKind::Keyword), Some("SEND"))?;

        let msg = self.parse_expr()?;

        self.eat(Some(TokenKind::Keyword), Some("TO"))?;

        let target = self.parse_expr()?;

        Ok(AstCall {
            func: "SEND".to_string(),
            args: vec![msg, target],
            line: send_token.line,
            column: send_token.column,
        })
    }

    pub fn parse_recv(&mut self) -> Result<AstCall, ParserError> {
        let recv_token = self.eat(Some(TokenKind::Keyword), Some("RECV"))?;

        let ident_token = self.eat(Some(TokenKind::Ident), None)?;

        Ok(AstCall {
            func: "RECV".to_string(),
            args: vec![AstExpr::Var(AstVar {
                name: ident_token.lexeme,
                line: ident_token.line,
                column: ident_token.column,
            })],
            line: recv_token.line,
            column: recv_token.column,
        })
    }

    pub fn parse_await_stmt(&mut self) -> Result<AstCall, ParserError> {
        let await_token = self.eat(Some(TokenKind::Keyword), Some("AWAIT"))?;

        let expr = self.parse_expr()?;

        Ok(AstCall {
            func: "AWAIT".to_string(),
            args: vec![expr],
            line: await_token.line,
            column: await_token.column,
        })
    }

    // Expressions

    pub fn parse_await_expr(&mut self) -> Result<AstCall, ParserError> {
        let await_token = self.eat(Some(TokenKind::Keyword), Some("AWAIT"))?;

        let expr = self.parse_expr()?;

        Ok(AstCall {
            func: "AWAIT".to_string(),
            args: vec![expr],
            line: await_token.line,
            column: await_token.column,
        })
    }

    pub fn parse_spawn_expr(&mut self) -> Result<AstCall, ParserError> {
        let spawn_token = self.eat(Some(TokenKind::Keyword), Some("SPAWN"))?;

        let target = self.parse_expr()?;

        Ok(AstCall {
            func: "SPAWN".to_string(),
            args: vec![target],
            line: spawn_token.line,
            column: spawn_token.column,
        })
    }

    pub fn parse_recv_expr(&mut self) -> Result<AstCall, ParserError> {
        let recv_token = self.eat(Some(TokenKind::Keyword), Some("RECV"))?;

        let source = self.parse_expr()?;

        Ok(AstCall {
            func: "RECV".to_string(),
            args: vec![source],
            line: recv_token.line,
            column: recv_token.column,
        })
    }

    pub fn parse_ping_expr(&mut self) -> Result<AstCall, ParserError> {
        let ping_token = self.eat(Some(TokenKind::Keyword), Some("PING"))?;

        let target = self.parse_expr()?;

        Ok(AstCall {
            func: "PING".to_string(),
            args: vec![target],
            line: ping_token.line,
            column: ping_token.column,
        })
    }

    pub fn parse_esc_expr(&mut self) -> Result<AstCall, ParserError> {
        let esc_token = self.eat(Some(TokenKind::Keyword), Some("ESC"))?;

        let target = self.parse_expr()?;

        Ok(AstCall {
            func: "ESC".to_string(),
            args: vec![target],
            line: esc_token.line,
            column: esc_token.column,
        })
    }

    pub fn parse_timeout_expr(&mut self) -> Result<AstCall, ParserError> {
        let timeout_token = self.eat(Some(TokenKind::Keyword), Some("TIMEOUT"))?;

        let target = self.parse_expr()?;
        let duration = self.parse_expr()?;

        Ok(AstCall {
            func: "TIMEOUT".to_string(),
            args: vec![target, duration],
            line: timeout_token.line,
            column: timeout_token.column,
        })
    }

    pub fn parse_task_expr(&mut self) -> Result<AstCall, ParserError> {
        let task_token = self.eat(Some(TokenKind::Keyword), Some("TASK"))?;

        self.eat(Some(TokenKind::LParen), None)?;

        let mut args = Vec::new();

        if !self.at(Some(TokenKind::RParen), None) {
            args.push(self.parse_expr()?);

            while self.at(Some(TokenKind::Comma), None) {
                self.eat(Some(TokenKind::Comma), None)?;
                args.push(self.parse_expr()?);
            }
        }

        self.eat(Some(TokenKind::RParen), None)?;

        Ok(AstCall {
            func: "TASK".to_string(),
            args,
            line: task_token.line,
            column: task_token.column,
        })
    }

    pub fn parse_sync_expr(&mut self) -> Result<AstCall, ParserError> {
        let sync_token = self.eat(Some(TokenKind::Keyword), Some("SYNC"))?;

        self.eat(Some(TokenKind::LParen), None)?;

        let mut args = Vec::new();

        if !self.at(Some(TokenKind::RParen), None) {
            args.push(self.parse_expr()?);

            while self.at(Some(TokenKind::Comma), None) {
                self.eat(Some(TokenKind::Comma), None)?;
                args.push(self.parse_expr()?);
            }
        }

        self.eat(Some(TokenKind::RParen), None)?;

        Ok(AstCall {
            func: "SYNC".to_string(),
            args,
            line: sync_token.line,
            column: sync_token.column,
        })
    }

    pub fn parse_channel_expr(&mut self) -> Result<AstCall, ParserError> {
        let channel_token = self.eat(Some(TokenKind::Keyword), Some("CHANNEL"))?;

        self.eat(Some(TokenKind::LParen), None)?;

        let mut args = Vec::new();

        if !self.at(Some(TokenKind::RParen), None) {
            let type_name = self.parse_type_ref()?;

            args.push(AstExpr::Var(AstVar {
                name: type_name,
                line: channel_token.line,
                column: channel_token.column,
            }));

            while self.at(Some(TokenKind::Comma), None) {
                self.eat(Some(TokenKind::Comma), None)?;

                let type_name = self.parse_type_ref()?;

                args.push(AstExpr::Var(AstVar {
                    name: type_name,
                    line: channel_token.line,
                    column: channel_token.column,
                }));
            }
        }

        self.eat(Some(TokenKind::RParen), None)?;

        Ok(AstCall {
            func: "CHANNEL".to_string(),
            args,
            line: channel_token.line,
            column: channel_token.column,
        })
    }

    pub fn parse_expr(&mut self) -> Result<AstExpr, ParserError> {
        self.parse_logic()
    }

    pub fn parse_logic(&mut self) -> Result<AstExpr, ParserError> {
        let mut left = self.parse_comp()?;

        while self.at(Some(TokenKind::Op), None) {
            let Some(token) = self.current() else {
                break;
            };

            if !matches!(token.lexeme.as_str(), "AND" | "OR") {
                break;
            }

            let op = self.eat(Some(TokenKind::Op), None)?.lexeme;

            let right = self.parse_comp()?;

            left = AstExpr::Binary(Box::new(AstBinary {
                kind: op,
                left: Box::new(left),
                right: Box::new(right),
            }));
        }

        Ok(left)
    }

    pub fn parse_comp(&mut self) -> Result<AstExpr, ParserError> {
        let mut left = self.parse_add()?;

        while self.at(Some(TokenKind::Op), None) {
            let op_name = match self.current() {
                Some(token) => token.lexeme.clone(),
                None => break,
            };

            if !matches!(
                op_name.as_str(),
                "EQ" | "NEQ" | "GT" | "LT" | "GTE" | "LTE" | "AS" | "IS"
            ) {
                break;
            }

            let op = self.eat(Some(TokenKind::Op), None)?.lexeme;

            let right = if op == "IS" || op == "AS" {
                if self.at(Some(TokenKind::LowType), None) {
                    let type_token = self.eat(Some(TokenKind::LowType), None)?;

                    AstExpr::Var(AstVar {
                        name: type_token.lexeme,
                        line: type_token.line,
                        column: type_token.column,
                    })
                } else {
                    let token = self.current().unwrap();

                    return Err(ParserError::new(
                        "NXD-P1012",
                        format!(
                            "NXD-P1012: Expected type after {}, got {:?} {}",
                            op, token.kind, token.lexeme
                        ),
                        token.span(),
                    ));
                }
            } else {
                self.parse_add()?
            };

            left = AstExpr::Binary(Box::new(AstBinary {
                kind: op,
                left: Box::new(left),
                right: Box::new(right),
            }));
        }

        Ok(left)
    }

    pub fn parse_add(&mut self) -> Result<AstExpr, ParserError> {
        let mut left = self.parse_mul()?;

        while self.at(Some(TokenKind::Op), None) {
            let Some(token) = self.current() else {
                break;
            };

            if !matches!(token.lexeme.as_str(), "ADD" | "SUB") {
                break;
            }

            let op = self.eat(Some(TokenKind::Op), None)?.lexeme;

            let right = self.parse_mul()?;

            left = AstExpr::Binary(Box::new(AstBinary {
                kind: op,
                left: Box::new(left),
                right: Box::new(right),
            }));
        }

        Ok(left)
    }

    pub fn parse_mul(&mut self) -> Result<AstExpr, ParserError> {
        let mut left = self.parse_unary()?;

        while self.at(Some(TokenKind::Op), None) {
            let Some(token) = self.current() else {
                break;
            };

            if !matches!(token.lexeme.as_str(), "MUL" | "DIV" | "MOD") {
                break;
            }

            let op = self.eat(Some(TokenKind::Op), None)?.lexeme;

            let right = self.parse_unary()?;

            left = AstExpr::Binary(Box::new(AstBinary {
                kind: op,
                left: Box::new(left),
                right: Box::new(right),
            }));
        }

        Ok(left)
    }

    pub fn parse_unary(&mut self) -> Result<AstExpr, ParserError> {
        if self.at(Some(TokenKind::Op), None) {
            if let Some(token) = self.current() {
                if matches!(token.lexeme.as_str(), "NOT" | "BORROW" | "SUB") {
                    let op = self.eat(Some(TokenKind::Op), None)?.lexeme;

                    let expr = self.parse_primary()?;

                    return Ok(AstExpr::Unary(Box::new(AstUnary {
                        kind: op,
                        expr: Box::new(expr),
                    })));
                }
            }
        }

        self.parse_primary()
    }

    pub fn parse_primary(&mut self) -> Result<AstExpr, ParserError> {
        let token = self.current().cloned().unwrap();

        if matches!(token.kind, TokenKind::Number | TokenKind::String) {
            return Ok(AstExpr::Literal(self.parse_literal()?));
        }

        if token.kind == TokenKind::LowName
            && matches!(token.lexeme.as_str(), "true" | "false" | "none")
        {
            return Ok(AstExpr::Literal(self.parse_literal()?));
        }

        if token.kind == TokenKind::LBrack {
            return Ok(AstExpr::Literal(self.parse_list_literal()?));
        }

        if token.kind == TokenKind::LBrace {
            return Ok(AstExpr::Literal(self.parse_map_literal()?));
        }

        if token.kind == TokenKind::Fn {
            return self.parse_lambda();
        }

        if token.kind == TokenKind::Keyword && token.lexeme == "AWAIT" {
            return Ok(AstExpr::Call(self.parse_await_expr()?));
        }

        if token.kind == TokenKind::Keyword && token.lexeme == "SPAWN" {
            return Ok(AstExpr::Call(self.parse_spawn_expr()?));
        }

        if token.kind == TokenKind::Keyword && token.lexeme == "RECV" {
            return Ok(AstExpr::Call(self.parse_recv_expr()?));
        }

        if token.kind == TokenKind::Keyword && token.lexeme == "PING" {
            return Ok(AstExpr::Call(self.parse_ping_expr()?));
        }

        if token.kind == TokenKind::Keyword && token.lexeme == "ESC" {
            return Ok(AstExpr::Call(self.parse_esc_expr()?));
        }

        if token.kind == TokenKind::Keyword && token.lexeme == "TIMEOUT" {
            return Ok(AstExpr::Call(self.parse_timeout_expr()?));
        }

        if token.kind == TokenKind::Keyword && token.lexeme == "TASK" {
            return Ok(AstExpr::Call(self.parse_task_expr()?));
        }

        if token.kind == TokenKind::Keyword && token.lexeme == "SYNC" {
            return Ok(AstExpr::Call(self.parse_sync_expr()?));
        }

        if token.kind == TokenKind::Keyword && token.lexeme == "CHANNEL" {
            return Ok(AstExpr::Call(self.parse_channel_expr()?));
        }

        if token.kind == TokenKind::Ident {
            if self.next_is(TokenKind::LParen) {
                return Ok(AstExpr::Call(self.parse_call_expr()?));
            }

            if self.next_is(TokenKind::LBrace) {
                return Ok(AstExpr::ObjectInit(self.parse_object_init()?));
            }

            let ident_token = self.eat(Some(TokenKind::Ident), None)?;

            return Ok(AstExpr::Var(AstVar {
                name: ident_token.lexeme,
                line: ident_token.line,
                column: ident_token.column,
            }));
        }

        Err(ParserError::new(
            "NXD-P1008",
            format!("NXD-P1008: Unexpected token in primary: {}", token.lexeme),
            token.span(),
        ))
    }

    pub fn parse_literal(&mut self) -> Result<AstLiteral, ParserError> {
        let token = self.current().expect("parse_literal called at EOF");

        if token.kind == TokenKind::Number {
            let value = self.eat(Some(TokenKind::Number), None)?;

            if value.lexeme.contains('.') {
                let parsed = value.lexeme.parse::<f64>().map_err(|_| {
                    ParserError::new(
                        "NXD-P1009-03",
                        format!("NXD-P1009-03: Invalid float literal: {}", value.lexeme),
                        value.span(),
                    )
                })?;

                if !parsed.is_finite() {
                    return Err(ParserError::new(
                        "NXD-P1009-03",
                        format!(
                            "NXD-P1009-03: Float literal exceeds finite range: {}",
                            value.lexeme
                        ),
                        value.span(),
                    ));
                }

                return Ok(AstLiteral::Float(parsed));
            }

            let parsed = value.lexeme.parse::<i64>().map_err(|_| {
                ParserError::new(
                    "NXD-P1009-02",
                    format!("NXD-P1009-02: Invalid integer literal: {}", value.lexeme),
                    value.span(),
                )
            })?;

            return Ok(AstLiteral::Int(parsed));
        }

        if token.kind == TokenKind::String {
            let value = self.eat(Some(TokenKind::String), None)?;

            return Ok(AstLiteral::String(
                value.lexeme[1..value.lexeme.len() - 1].to_string(),
            ));
        }

        if token.kind == TokenKind::LowName {
            let value = self.eat(Some(TokenKind::LowName), None)?;

            match value.lexeme.as_str() {
                "true" => {
                    return Ok(AstLiteral::Bool(true));
                }

                "false" => {
                    return Ok(AstLiteral::Bool(false));
                }

                "none" => {
                    return Ok(AstLiteral::None);
                }

                _ => {}
            }
        }

        let token = self.current().unwrap();

        Err(ParserError::new(
            "NXD-P1009-01",
            format!(
                "NXD-P1009-01: Literal expected, got {:?} {}",
                token.kind, token.lexeme
            ),
            token.span(),
        ))
    }

    pub fn parse_list_literal(&mut self) -> Result<AstLiteral, ParserError> {
        self.eat(Some(TokenKind::LBrack), None)?;

        let mut items: Vec<AstExpr> = Vec::new();

        while !self.at(Some(TokenKind::RBrack), None) {
            items.push(self.parse_expr()?);

            if self.at(Some(TokenKind::Comma), None) {
                self.eat(Some(TokenKind::Comma), None)?;
            } else if !self.at(Some(TokenKind::RBrack), None) {
                let token = self.current().unwrap();

                return Err(ParserError::new(
                    "NXD-P1009-04",
                    format!(
                        "NXD-P1009-04: Invalid list literal, expected ',' or ']', got {:?} {}",
                        token.kind, token.lexeme
                    ),
                    token.span(),
                ));
            }
        }

        self.eat(Some(TokenKind::RBrack), None)?;

        Ok(AstLiteral::List(items))
    }

    pub fn parse_object_init(&mut self) -> Result<AstObjectInit, ParserError> {
        let type_name = self.eat(Some(TokenKind::Ident), None)?.lexeme;

        let map_literal = self.parse_map_literal()?;

        match map_literal {
            AstLiteral::Map(fields) => Ok(AstObjectInit { type_name, fields }),

            _ => unreachable!(),
        }
    }

    pub fn parse_map_literal(&mut self) -> Result<AstLiteral, ParserError> {
        self.eat(Some(TokenKind::LBrace), None)?;

        let mut entries: HashMap<String, AstExpr> = HashMap::new();

        while !self.at(Some(TokenKind::RBrace), None) {
            while self.at(Some(TokenKind::Newline), None) {
                self.eat(Some(TokenKind::Newline), None)?;
            }

            // Stop if blank lines brought us to the closing brace.
            if self.at(Some(TokenKind::RBrace), None) {
                break;
            }

            let key = if self.at(Some(TokenKind::Ident), None) {
                self.eat(Some(TokenKind::Ident), None)?.lexeme
            } else if self.at(Some(TokenKind::LowName), None) {
                self.eat(Some(TokenKind::LowName), None)?.lexeme
            } else if self.at(Some(TokenKind::String), None) {
                let string_token = self.eat(Some(TokenKind::String), None)?;

                string_token.lexeme[1..string_token.lexeme.len() - 1].to_string()
            } else {
                let token = self
                    .current()
                    .cloned()
                    .expect("parse_map_literal called at EOF");

                return Err(ParserError::new(
                    "NXD-P1010",
                    format!(
                        "NXD-P1010: Expected map key, got {:?} {}",
                        token.kind, token.lexeme,
                    ),
                    token.span(),
                ));
            };

            self.eat(Some(TokenKind::Colon), None)?;

            let value = self.parse_expr()?;

            entries.insert(key, value);

            if self.at(Some(TokenKind::Comma), None) {
                self.eat(Some(TokenKind::Comma), None)?;
            } else if !self.at(Some(TokenKind::RBrace), None)
                && !self.at(Some(TokenKind::Newline), None)
            {
                let token = self.current().unwrap();

                return Err(ParserError::new(
                    "NXD-P1009-05",
                    format!(
                        "NXD-P1009-05: Invalid map literal, expected ',' or '}}', got {:?} {}",
                        token.kind, token.lexeme
                    ),
                    token.span(),
                ));
            }

            while self.at(Some(TokenKind::Newline), None) {
                self.eat(Some(TokenKind::Newline), None)?;
            }
        }

        self.eat(Some(TokenKind::RBrace), None)?;

        Ok(AstLiteral::Map(entries))
    }

    pub fn parse_lambda(&mut self) -> Result<AstExpr, ParserError> {
        self.eat(Some(TokenKind::Fn), None)?;

        self.eat(Some(TokenKind::LParen), None)?;

        let mut params = Vec::new();

        if !self.at(Some(TokenKind::RParen), None) {
            params.push(self.eat(Some(TokenKind::Ident), None)?.lexeme);

            while !self.at(Some(TokenKind::RParen), None) {
                if self.at(Some(TokenKind::Comma), None) {
                    self.eat(Some(TokenKind::Comma), None)?;

                    params.push(self.eat(Some(TokenKind::Ident), None)?.lexeme);
                } else {
                    let token = self.current().unwrap();

                    return Err(ParserError::new(
                        "NXD-P1009-06",
                        format!(
                            "NXD-P1009-06: Invalid lambda literal, expected ',' or ')', got {:?} {}",
                            token.kind, token.lexeme
                        ),
                        token.span(),
                    ));
                }
            }
        }

        self.eat(Some(TokenKind::RParen), None)?;

        self.eat(Some(TokenKind::Arrow), None)?;

        let body = self.parse_expr()?;

        Ok(AstExpr::Literal(AstLiteral::Lambda {
            params,
            body: Box::new(body),
        }))
    }

    pub fn parse_call_expr(&mut self) -> Result<AstCall, ParserError> {
        let ident_token = self.eat(Some(TokenKind::Ident), None)?;

        let name = ident_token.lexeme.clone();

        self.eat(Some(TokenKind::LParen), None)?;

        let mut args = Vec::new();

        if !self.at(Some(TokenKind::RParen), None) {
            args.push(self.parse_expr()?);

            while self.at(Some(TokenKind::Comma), None) {
                self.eat(Some(TokenKind::Comma), None)?;

                args.push(self.parse_expr()?);
            }
        }

        self.eat(Some(TokenKind::RParen), None)?;

        Ok(AstCall {
            func: name,
            args,
            line: ident_token.line,
            column: ident_token.column,
        })
    }
}

pub fn parse(src: &str) -> Result<(AstModule, Vec<AstNode>, Vec<AstFunction>), ParserError> {
    let mut parser = Parser::new(src, false).map_err(|message| {
        ParserError::new(
            "NXD-P1099",
            message,
            SourceSpan {
                line: 0,
                column: 0,
                end_line: 0,
                end_column: 0,
            },
        )
    })?;

    let module = parser.parse_program()?;

    let mut ast_types = Vec::new();
    let mut ast_functions = Vec::new();
    let mut ast_statements = Vec::new();

    for node in &module.body {
        match node {
            AstNode::Struct(_)
            | AstNode::Enum(_)
            | AstNode::Union(_)
            | AstNode::Trait(_)
            | AstNode::Impl(_) => {
                ast_types.push(node.clone());
            }

            AstNode::Function(function) => {
                ast_functions.push(function.clone());
            }

            AstNode::Statement(statement) => {
                ast_statements.push(statement.clone());
            }
        }
    }

    Ok((module, ast_types, ast_functions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_parser_from_valid_source() {
        let parser = Parser::new("LET VALUE 42\nRETURN VALUE", false)
            .expect("Parser should accept valid lexer input");

        assert_eq!(parser.pos, 0);
        assert_eq!(parser.indent_stack, vec![0]);
        assert!(parser.errors.is_empty());
        assert!(!parser.recover_errors);
        assert!(!parser.tokens.is_empty());
    }

    #[test]
    fn propagates_lexer_errors() {
        let result = Parser::new("LET VALUE @", false);

        assert!(result.is_err());
    }
}
