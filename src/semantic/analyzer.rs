use crate::semantic::{
    symbols::{SymbolTable, Symbol},
    types::check_type,
    traits::TraitRegistry,
    casts::{check_as_cast, check_is},
    ownership::{check_ownership, OwnershipOp},
    patterns::validate_match_arms,
    errors::SemanticError,
};
use crate::ir::nodes::*;
use crate::ir::parse_ir::IRRoot;


pub struct Analyzer {
    pub symbols: SymbolTable,
    pub traits: TraitRegistry,
}

impl Analyzer {
    pub fn new(traits: TraitRegistry) -> Self {

        let mut symbols = SymbolTable::new();

        symbols.define(
       "PRINTLN",
        Symbol::Func {
                name: "PRINTLN".to_string(),
                params: vec!["any".to_string()],
                ret: Some("none".to_string()),
            },
        );

        Self {
            symbols,
            traits,
        }
    }
    pub fn analyze(&mut self, ir: &IRRoot) -> Result<(), SemanticError> {
        self.analyze_module(&ir.module)?;
        self.analyze_types(&ir.types)?;
        self.analyze_functions(&ir.functions)?;
        Ok(())
    }

    fn analyze_module(&mut self, module: &IRModule) -> Result<(), SemanticError> {
        self.symbols.define(&module.name, Symbol::Type { name: module.name.clone() });
        Ok(())
    }

    fn analyze_types(&mut self, types: &Vec<IRTypeDecl>) -> Result<(), SemanticError> {
        for t in types {
            match t {
                IRTypeDecl::Struct(s) => {
                    self.symbols.define(&s.name, Symbol::Type { name: s.name.clone() });
                }
                IRTypeDecl::Enum(e) => {
                    self.symbols.define(&e.name, Symbol::Type { name: e.name.clone() });
                }
                IRTypeDecl::Union(u) => {
                    self.symbols.define(&u.name, Symbol::Type { name: u.name.clone() });
                }
                IRTypeDecl::Trait(tr) => {
                    self.symbols.define(&tr.name, Symbol::Trait { name: tr.name.clone() });
                }
            }
        }
        Ok(())
    }

    fn analyze_functions(&mut self, funcs: &Vec<IRFunction>) -> Result<(), SemanticError> {
        for f in funcs {
            self.symbols.define(
                &f.name,
                Symbol::Func {
                    name: f.name.clone(),
                    params: f.params.iter().map(|p| p.ty.clone()).collect(),
                    ret: f.return_type.clone(),
                },
            );

            self.symbols.enter_scope();
            for p in &f.params {
                self.symbols.define(&p.name, Symbol::Var { name: p.name.clone(), ty: p.ty.clone() });
            }

            for stmt in &f.body {
                self.analyze_statement(stmt)?;
            }

            self.symbols.exit_scope();
        }
        Ok(())
    }

    fn analyze_statement(&mut self, stmt: &IRStatement) -> Result<(), SemanticError> {
        match stmt {
            IRStatement::Let { name, value } => {
                let ty = self.analyze_expr(value)?;
                self.symbols.define(name, Symbol::Var { name: name.clone(), ty });
            }
            IRStatement::Const { name, value } => {
                let ty = self.analyze_expr(value)?;
                self.symbols.define(name, Symbol::Const { name: name.clone(), ty });
            }
            IRStatement::Return(expr) => {
                self.analyze_expr(expr)?;
            }
            IRStatement::Loop(body) => {
                for s in body {
                    self.analyze_statement(s)?;
                }
            }
            IRStatement::If(if_node) => {
                self.analyze_expr(&if_node.condition)?;
                for s in &if_node.then_branch {
                    self.analyze_statement(s)?;
                }
                for s in &if_node.else_branch {
                    self.analyze_statement(s)?;
                }
            }
            IRStatement::Match(m) => {
                self.analyze_expr(&m.scrutinee)?;
                validate_match_arms(&m.arms)?;
            }
            IRStatement::Try(t) => {
                for s in &t.try_body {
                    self.analyze_statement(s)?;
                }

                for s in &t.catch_body {
                    self.analyze_statement(s)?;
                }

                for s in &t.finally_body {
                    self.analyze_statement(s)?;
                }
            }
            IRStatement::Move(move_node) => {
                let source_type = self.analyze_expr(&move_node.source)?;

                let target_name = match &move_node.target {
                    IRExpr::Var(var) => var.name.clone(),
                    _ => {
                        return Err(SemanticError::UndefinedSymbol {
                            name: "MOVE target must be an identifier".to_string(),
                            line: 1,
                            column: 1,
                            end_line: 1,
                            end_column: 2,
                        });
                    }
                };

                check_ownership(OwnershipOp::Move, &source_type)?;

                self.symbols.define(
                    &target_name,
                    Symbol::Var {
                        name: target_name.clone(),
                        ty: source_type,
                    },
                );
            }
            IRStatement::Clone(clone_node) => {
                let source_type = self.analyze_expr(&clone_node.source)?;

                let target_name = match &clone_node.target {
                    IRExpr::Var(var) => var.name.clone(),
                    _ => {
                        return Err(SemanticError::UndefinedSymbol {
                            name: "CLONE target must be an identifier".to_string(),
                            line: 1,
                            column: 1,
                            end_line: 1,
                            end_column: 2,
                        });
                    }
                };

                check_ownership(OwnershipOp::Clone, &source_type)?;

                self.symbols.define(
                    &target_name,
                    Symbol::Var {
                        name: target_name.clone(),
                        ty: source_type,
                    },
                );
            }
            IRStatement::Expr(expr) => {
                self.analyze_expr(expr)?;
            }
        }
        Ok(())
    }

    fn analyze_expr(
        &mut self,
        expr: &IRExpr,
    ) -> Result<String, SemanticError> {
        match expr {
            IRExpr::Literal(literal) => {
                Ok(self.literal_type(literal))
            }

            IRExpr::ObjectInit(init) => {
                for value in init.fields.values() {
                    self.analyze_expr(value)?;
                }

                Ok(init.type_name.clone())
            }

            IRExpr::Var(var) => {
                if let Some(symbol) =
                    self.symbols.resolve(&var.name)
                {
                    match symbol {
                        Symbol::Var { ty, .. } => {
                            Ok(ty.clone())
                        }

                        Symbol::Const { ty, .. } => {
                            Ok(ty.clone())
                        }

                        _ => {
                            Err(
                                SemanticError::UndefinedSymbol {
                                    name: var.name.clone(),
                                    line: var.span.line,
                                    column: var.span.column,
                                    end_line: var.span.end_line,
                                    end_column: var.span.end_column,
                                }
                            )
                        }
                    }
                } else {
                    Err(
                        SemanticError::UndefinedSymbol {
                            name: var.name.clone(),
                            line: var.span.line,
                            column: var.span.column,
                            end_line: var.span.end_line,
                            end_column: var.span.end_column,

                        }
                    )
                }
            }

            IRExpr::Binary(binary) => {
                let left =
                    self.analyze_expr(&binary.left)?;

                match binary.kind.as_str() {
                    "AS" => {
                        let target_type =
                            match binary.right.as_ref() {
                                IRExpr::Var(var) => {
                                    var.name.clone()
                                }

                                _ => {
                                    return Err(
                                        SemanticError::UndefinedSymbol {
                                            name: (
                                                "Expected type name after AS"
                                            ).to_string(),
                                            line: 1,
                                            column: 1,
                                            end_line: 1,
                                            end_column: 2,
                                        }
                                    );
                                }
                            };

                        check_as_cast(
                            &left,
                            &target_type,
                        )?;

                        Ok(target_type)
                    }

                    "IS" => {
                        let target_type =
                            match binary.right.as_ref() {
                                IRExpr::Var(var) => {
                                    var.name.clone()
                                }

                                _ => {
                                    return Err(
                                        SemanticError::UndefinedSymbol {
                                            name: (
                                                "Expected type name after IS"
                                            ).to_string(),
                                            line: 1,
                                            column: 1,
                                            end_line: 1,
                                            end_column: 2,
                                        }
                                    );
                                }
                            };

                        let _ = check_is(
                            &left,
                            &target_type,
                        );

                        Ok("bool".to_string())
                    }

                    _ => {
                        let right =
                            self.analyze_expr(
                                &binary.right
                            )?;

                        check_type(
                            &left,
                            &right,
                        )?;

                        Ok(left)
                    }
                }
            }

            IRExpr::Unary(unary) => {
                let inner =
                    self.analyze_expr(&unary.expr)?;

                match unary.kind.as_str() {
                    "MOVE" => {
                        check_ownership(
                            OwnershipOp::Move,
                            &inner,
                        )?;

                        Ok(inner)
                    }

                    "CLONE" => {
                        check_ownership(
                            OwnershipOp::Clone,
                            &inner,
                        )?;

                        Ok(inner)
                    }

                    "BORROW" => {
                        check_ownership(
                            OwnershipOp::Borrow,
                            &inner,
                        )?;

                        Ok(inner)
                    }

                    _ => Ok(inner),
                }
            }

            IRExpr::Call { func, args } => {
                let (params, ret) = {
                    let symbol = self.symbols
                        .resolve(func)
                        .ok_or_else(|| {
                            SemanticError::UndefinedSymbol {
                                name: func.clone(),
                                line: 1,
                                column: 1,
                                end_line: 1,
                                end_column: 2,
                            }
                        })?;

                    if let Symbol::Func {
                        params,
                        ret,
                        ..
                    } = symbol
                    {
                        (
                            params.clone(),
                            ret.clone(),
                        )
                    } else {
                        return Err(
                            SemanticError::UndefinedSymbol {
                                name: func.clone(),
                                line: 1,
                                column: 1,
                                end_line: 1,
                                end_column: 2,
                            }
                        );
                    }
                };

                for (index, argument)
                    in args.iter().enumerate()
                {
                    let argument_type =
                        self.analyze_expr(argument)?;

                    if index >= params.len() {
                        return Err(
                            SemanticError::UndefinedSymbol {
                                name: func.clone(),
                                line: 1,
                                column: 1,
                                end_line: 1,
                                end_column: 2,
                            }
                        );
                    }

                    check_type(
                        &params[index],
                        &argument_type,
                    )?;
                }

                Ok(
                    ret.unwrap_or(
                        "none".to_string()
                    )
                )
            }

            IRExpr::Pipeline { value, func } => {
                let value_type =
                    self.analyze_expr(value)?;

                let (params, ret) = {
                    let symbol = self.symbols
                        .resolve(func)
                        .ok_or_else(|| {
                            SemanticError::UndefinedSymbol {
                                name: func.clone(),
                                line: 1,
                                column: 1,
                                end_line: 1,
                                end_column: 2,
                            }
                        })?;

                    if let Symbol::Func {
                        params,
                        ret,
                        ..
                    } = symbol
                    {
                        (
                            params.clone(),
                            ret.clone(),
                        )
                    } else {
                        return Err(
                            SemanticError::UndefinedSymbol {
                                name: func.clone(),
                                line: 1,
                                column: 1,
                                end_line: 1,
                                end_column: 2,
                            }
                        );
                    }
                };

                if params.is_empty() {
                    return Err(
                        SemanticError::UndefinedSymbol {
                            name: func.clone(),
                            line: 1,
                            column: 1,
                            end_line: 1,
                            end_column: 2,
                        }
                    );
                }

                check_type(
                    &params[0],
                    &value_type,
                )?;

                Ok(
                    ret.unwrap_or(
                        "none".to_string()
                    )
                )
            }
        }
    }

    fn literal_type(
        &self,
        literal: &IRLiteral,
    ) -> String {
        match literal {
            IRLiteral::Int(_) => {
                "int".to_string()
            }

            IRLiteral::Float(_) => {
                "float".to_string()
            }

            IRLiteral::String(_) => {
                "string".to_string()
            }

            IRLiteral::Bool(_) => {
                "bool".to_string()
            }

            IRLiteral::None => {
                "none".to_string()
            }

            IRLiteral::List(_) => {
                "list".to_string()
            }
        }
    }
    }