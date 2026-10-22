use crate::diagnostics::severity::DiagnosticSeverity;
use crate::diagnostics::span::SourceSpan;
use crate::diagnostics::types::ParserError;

use crate::frontend::ast::nodes::{AstFunction, AstModule, AstNode};

use crate::frontend::parser::parser::Parser;

impl ParserError {
    pub fn new(code: impl Into<String>, message: impl Into<String>, span: SourceSpan) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            span,
            severity: DiagnosticSeverity::Error,
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for ParserError {}

pub fn parse_with_diagnostics(
    src: &str,
) -> Result<(AstModule, Vec<AstNode>, Vec<AstFunction>, Vec<ParserError>), ParserError> {
    let mut parser = Parser::new(src, true).map_err(|message| {
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

    Ok((module, ast_types, ast_functions, parser.errors))
}
