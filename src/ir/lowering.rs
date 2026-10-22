use crate::diagnostics::span::SourceSpan;
use crate::frontend::ast::nodes::*;
use crate::ir::nodes::*;

fn lower_span(span: &SourceSpan) -> SourceSpan {
    span.clone()
}

pub fn lower_module(ast: &AstModule) -> IRModule {
    IRModule {
        name: ast.name.clone(),
        imports: ast.imports.iter().map(lower_import).collect(),
    }
}

pub fn lower_import(ast: &AstImport) -> IRImport {
    IRImport {
        path: ast.path.clone(),
        alias: ast.alias.clone(),
    }
}

pub fn lower_types(ast_nodes: &[AstNode]) -> Vec<IRTypeDecl> {
    let mut out = Vec::new();

    for node in ast_nodes {
        match node {
            AstNode::Struct(s) => {
                out.push(IRTypeDecl::Struct(lower_struct(s)));
            }

            AstNode::Enum(e) => {
                out.push(IRTypeDecl::Enum(lower_enum(e)));
            }

            AstNode::Union(u) => {
                out.push(IRTypeDecl::Union(lower_union(u)));
            }

            AstNode::Trait(t) => {
                out.push(IRTypeDecl::Trait(lower_trait(t)));
            }

            // Python ignores everything else.
            AstNode::Impl(_) | AstNode::Function(_) | AstNode::Statement(_) => {}
        }
    }

    out
}

pub fn lower_struct(ast: &AstStruct) -> IRStruct {
    IRStruct {
        name: ast.name.clone(),

        fields: ast
            .fields
            .iter()
            .map(|field| IRField {
                name: field.name.clone(),
                ty: field.ty.clone(),
            })
            .collect(),
    }
}

pub fn lower_enum(ast: &AstEnum) -> IREnum {
    IREnum {
        name: ast.name.clone(),
        variants: ast.variants.clone(),
    }
}

pub fn lower_union(ast: &AstUnion) -> IRUnion {
    IRUnion {
        name: ast.name.clone(),

        variants: ast
            .variants
            .iter()
            .map(|variant| IRUnionVariant {
                kind: variant.kind.clone(),

                fields: variant
                    .fields
                    .iter()
                    .map(|field| IRField {
                        name: field.name.clone(),
                        ty: field.ty.clone(),
                    })
                    .collect(),
            })
            .collect(),
    }
}

pub fn lower_trait(ast: &AstTrait) -> IRTrait {
    IRTrait {
        name: ast.name.clone(),

        methods: ast
            .methods
            .iter()
            .map(|method| IRFunctionSignature {
                name: method.name.clone(),

                params: method
                    .params
                    .iter()
                    .map(|param| IRParam {
                        name: param.name.clone(),
                        ty: "any".to_string(),
                    })
                    .collect(),

                return_type: method.return_type.clone(),
            })
            .collect(),
    }
}

pub fn lower_function(ast: &AstFunction) -> IRFunction {
    IRFunction {
        name: ast.name.clone(),
        span: ast.span.clone(),

        params: ast
            .params
            .iter()
            .map(|param| IRParam {
                name: param.name.clone(),
                ty: "any".to_string(),
            })
            .collect(),

        return_type: ast.return_type.clone(),
        body: ast.body.iter().map(lower_statement).collect(),

        is_async: ast.is_async,
    }
}

pub fn lower_statement(ast: &AstStatement) -> IRStatement {
    match ast {
        AstStatement::Let(stmt) => IRStatement::Let {
            name: stmt.name.clone(),
            value: lower_expr(&stmt.value),
        },

        AstStatement::Const(stmt) => IRStatement::Const {
            name: stmt.name.clone(),
            value: lower_expr(&stmt.value),
        },

        AstStatement::Return(stmt) => IRStatement::Return(lower_expr(&stmt.value)),

        AstStatement::Loop(stmt) => {
            IRStatement::Loop(stmt.body.iter().map(lower_statement).collect())
        }

        AstStatement::If(stmt) => IRStatement::If(IRIf {
            condition: lower_expr(&stmt.condition),

            then_branch: stmt.then_branch.iter().map(lower_statement).collect(),

            else_branch: stmt.else_branch.iter().map(lower_statement).collect(),
        }),

        AstStatement::Try(stmt) => IRStatement::Try(IRTry {
            try_body: stmt.try_body.iter().map(lower_statement).collect(),

            catch_body: stmt.catch_body.iter().map(lower_statement).collect(),

            finally_body: stmt.finally_body.iter().map(lower_statement).collect(),
        }),

        AstStatement::Move(stmt) => IRStatement::Move(IRMove {
            source: lower_expr(&stmt.source),

            target: lower_expr(&stmt.target),
        }),

        AstStatement::Clone(stmt) => IRStatement::Clone(IRClone {
            source: lower_expr(&stmt.source),

            target: lower_expr(&stmt.target),
        }),

        AstStatement::Match(stmt) => IRStatement::Match(IRMatch {
            scrutinee: lower_expr(&stmt.scrutinee),

            arms: stmt
                .arms
                .iter()
                .map(|arm| IRMatchArm {
                    pattern: arm.pattern.clone(),

                    body: arm.body.iter().map(lower_statement).collect(),
                })
                .collect(),

            otherwise: stmt
                .otherwise
                .as_ref()
                .map(|otherwise| otherwise.iter().map(lower_statement).collect()),
        }),

        AstStatement::Expr(expr) => IRStatement::Expr(lower_expr(expr)),
    }
}

pub fn lower_expr(ast: &AstExpr) -> IRExpr {
    match ast {
        AstExpr::Literal(lit) => IRExpr::Literal(lower_literal(lit)),

        AstExpr::Binary(op) => IRExpr::Binary(Box::new(IRBinaryOp {
            kind: op.kind.clone(),

            left: Box::new(lower_expr(&op.left)),

            right: Box::new(lower_expr(&op.right)),
        })),

        AstExpr::Unary(op) => IRExpr::Unary(Box::new(IRUnaryOp {
            kind: op.kind.clone(),

            expr: Box::new(lower_expr(&op.expr)),
        })),

        AstExpr::Call(call) => IRExpr::Call {
            func: call.func.clone(),

            args: call.args.iter().map(lower_expr).collect(),
        },

        AstExpr::Var(var) => IRExpr::Var(IRVar {
            name: var.name.clone(),

            span: SourceSpan {
                line: var.line,
                column: var.column,
                end_line: var.line,
                end_column: var.column + var.name.len() as u32,
            },
        }),
        AstExpr::Pipeline(pipe) => IRExpr::Pipeline {
            value: Box::new(lower_expr(&pipe.value)),

            func: pipe.func.clone(),
        },

        AstExpr::ObjectInit(obj) => IRExpr::ObjectInit(IRObjectInit {
            type_name: obj.type_name.clone(),

            fields: obj
                .fields
                .iter()
                .map(|(k, v)| (k.clone(), lower_expr(v)))
                .collect(),
        }),
    }
}

pub fn lower_literal(ast: &AstLiteral) -> IRLiteral {
    match ast {
        AstLiteral::Int(value) => IRLiteral::Int(*value),

        AstLiteral::Float(value) => IRLiteral::Float(*value),

        AstLiteral::String(value) => IRLiteral::String(value.clone()),

        AstLiteral::Bool(value) => IRLiteral::Bool(*value),

        AstLiteral::None => IRLiteral::None,

        AstLiteral::List(items) => IRLiteral::List(items.iter().map(lower_expr).collect()),

        AstLiteral::Map(entries) => IRLiteral::Map(
            entries
                .iter()
                .map(|(key, value)| (key.clone(), lower_expr(value)))
                .collect(),
        ),

        AstLiteral::Lambda { params, body } => IRLiteral::Lambda {
            params: params.clone(),
            body: Box::new(lower_expr(body)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn lowers_integer_literal() {
        let ast = AstLiteral::Int(42);

        let ir = lower_literal(&ast);

        match ir {
            IRLiteral::Int(value) => {
                assert_eq!(value, 42);
            }
            _ => panic!("Expected IRLiteral::Int"),
        }
    }

    #[test]
    fn lowers_map_literal() {
        let mut map = HashMap::new();

        map.insert(
            "name".to_string(),
            AstExpr::Literal(AstLiteral::String("gabby".to_string())),
        );

        let ast = AstLiteral::Map(map);

        let ir = lower_literal(&ast);

        match ir {
            IRLiteral::Map(entries) => {
                assert_eq!(entries.len(), 1);
                assert!(entries.contains_key("name"));
            }
            _ => panic!("Expected IRLiteral::Map"),
        }
    }

    #[test]
    fn lowers_lambda_literal() {
        let ast = AstLiteral::Lambda {
            params: vec!["x".to_string(), "y".to_string()],

            body: Box::new(AstExpr::Literal(AstLiteral::Int(42))),
        };

        let ir = lower_literal(&ast);

        match ir {
            IRLiteral::Lambda { params, body } => {
                assert_eq!(params.len(), 2);

                match *body {
                    IRExpr::Literal(IRLiteral::Int(42)) => {}
                    _ => panic!("Expected lambda body to be Int(42)"),
                }
            }

            _ => panic!("Expected IRLiteral::Lambda"),
        }
    }

    #[test]
    fn lowers_var_preserves_span() {
        let ast = AstExpr::Var(AstVar {
            name: "TEST".to_string(),
            line: 10,
            column: 5,
        });

        let ir = lower_expr(&ast);

        match ir {
            IRExpr::Var(var) => {
                assert_eq!(var.name, "TEST");
                assert_eq!(var.span.line, 10);
                assert_eq!(var.span.column, 5);
                assert_eq!(var.span.end_line, 10);
                assert_eq!(var.span.end_column, 9);
            }

            _ => panic!("Expected IRExpr::Var"),
        }
    }

    #[test]
    fn lowers_function_metadata() {
        let ast = AstFunction {
            name: "TEST".to_string(),

            span: SourceSpan {
                line: 1,
                column: 1,
                end_line: 1,
                end_column: 10,
            },

            params: vec![AstParam {
                name: "x".to_string(),
                ty: "INT".to_string(),
            }],

            return_type: Some("INT".to_string()),

            body: vec![],

            is_async: true,
        };

        let ir = lower_function(&ast);

        assert_eq!(ir.name, "TEST");

        assert_eq!(ir.return_type, Some("INT".to_string()));

        assert!(ir.is_async);

        assert_eq!(ir.params.len(), 1);

        assert_eq!(ir.params[0].name, "x");
    }
}
