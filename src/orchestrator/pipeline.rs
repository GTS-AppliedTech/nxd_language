use std::collections::HashMap;

use crate::backend::nim::emitter;
use crate::diagnostics::diagnostics::{SemanticDiagnostic, SemanticDiagnosticResponse};
use crate::diagnostics::errors::SemanticError;
use crate::ir::nodes::{IRExpr, IRLiteral, IRRoot, IRStatement, IRTypeDecl};
use crate::semantic::analyzer::Analyzer;
use crate::semantic::symbols::Symbol;
use crate::semantic::tooling::{ToolingAnalysis, ToolingMember, ToolingSymbol};
use crate::semantic::traits::TraitRegistry;

fn analyze_ir(ir_root: &IRRoot) -> Result<(), SemanticError> {
    let traits = TraitRegistry::from_ir(&ir_root.traits, &ir_root.impls);
    let mut analyzer = Analyzer::new(traits);

    analyzer.analyze(ir_root)
}

fn analyze_ir_with_symbols(ir_root: &IRRoot) -> Result<Analyzer, SemanticError> {
    let traits = TraitRegistry::from_ir(&ir_root.traits, &ir_root.impls);
    let mut analyzer = Analyzer::new(traits);

    analyzer.analyze(ir_root)?;

    Ok(analyzer)
}

pub fn definition_from_ir(ir_root: &IRRoot, symbol: &str) -> Result<String, String> {
    let analyzer = analyze_ir_with_symbols(ir_root).map_err(|error| error.message())?;

    match analyzer.symbols.resolve(symbol) {
        Some(Symbol::Func {
            span: Some(span), ..
        }) => Ok(format!(
            "{}:{}:{}:{}",
            span.line, span.column, span.end_line, span.end_column,
        )),

        Some(_) => Err(format!("'{}' is not a function definition", symbol,)),

        None => Err(format!("Definition not found for '{}'", symbol,)),
    }
}

pub fn compile_from_ir(ir_root: &IRRoot) -> Result<String, String> {
    let nim_code = emitter::emit(ir_root);

    Ok(nim_code)
}

pub fn semantic_diagnostics_from_ir(ir_root: &IRRoot) -> SemanticDiagnosticResponse {
    match analyze_ir(ir_root) {
        Ok(()) => SemanticDiagnosticResponse {
            ok: true,
            diagnostics: Vec::new(),
        },

        Err(error) => SemanticDiagnosticResponse {
            ok: false,

            diagnostics: vec![SemanticDiagnostic {
                code: error.code().to_string(),

                message: error.message(),

                line: error.line() as usize,

                column: error.column() as usize,

                end_line: error.end_line() as usize,

                end_column: error.end_column() as usize,

                severity: "error".to_string(),
            }],
        },
    }
}

fn collect_members_from_expr(
    expr: &IRExpr,
    members: &mut Vec<ToolingMember>,
    variables: &HashMap<String, ToolingSymbol>,
    constants: &HashMap<String, ToolingSymbol>,
    types: &HashMap<String, HashMap<String, String>>,
) {
    match expr {
        IRExpr::MemberAccess(access) => {
            collect_members_from_expr(&access.object, members, variables, constants, types);

            if let IRExpr::Var(var) = access.object.as_ref() {
                let object_type = variables
                    .get(&var.name)
                    .or_else(|| constants.get(&var.name))
                    .map(|symbol| &symbol.ty);

                if let Some(object_type) = object_type {
                    if let Some(fields) = types.get(object_type) {
                        if let Some((_, field_type)) = fields
                            .iter()
                            .find(|(name, _)| name.eq_ignore_ascii_case(&access.member))
                        {
                            members.push(ToolingMember {
                                object_type: object_type.clone(),
                                member: access.member.clone(),
                                ty: field_type.clone(),
                                span: access.span.clone(),
                            });
                        }
                    }
                }
            }
        }

        IRExpr::Binary(binary) => {
            collect_members_from_expr(&binary.left, members, variables, constants, types);
            collect_members_from_expr(&binary.right, members, variables, constants, types);
        }

        IRExpr::Unary(unary) => {
            collect_members_from_expr(&unary.expr, members, variables, constants, types);
        }

        IRExpr::Call { args, .. } => {
            for arg in args {
                collect_members_from_expr(arg, members, variables, constants, types);
            }
        }

        IRExpr::Pipeline { value, .. } => {
            collect_members_from_expr(value, members, variables, constants, types);
        }

        IRExpr::ObjectInit(init) => {
            for value in init.fields.values() {
                collect_members_from_expr(value, members, variables, constants, types);
            }
        }

        IRExpr::Literal(IRLiteral::List(items)) => {
            for item in items {
                collect_members_from_expr(item, members, variables, constants, types);
            }
        }

        IRExpr::Literal(IRLiteral::Map(entries)) => {
            for value in entries.values() {
                collect_members_from_expr(value, members, variables, constants, types);
            }
        }

        IRExpr::Literal(IRLiteral::Lambda { body, .. }) => {
            collect_members_from_expr(body, members, variables, constants, types);
        }

        IRExpr::Literal(_) | IRExpr::Var(_) => {}
    }
}

fn collect_members_from_statement(
    stmt: &IRStatement,
    members: &mut Vec<ToolingMember>,
    variables: &HashMap<String, ToolingSymbol>,
    constants: &HashMap<String, ToolingSymbol>,
    types: &HashMap<String, HashMap<String, String>>,
) {
    match stmt {
        IRStatement::Let { value, .. }
        | IRStatement::Const { value, .. }
        | IRStatement::Return(value)
        | IRStatement::Expr(value) => {
            collect_members_from_expr(value, members, variables, constants, types);
        }

        IRStatement::Loop(body) => {
            for statement in body {
                collect_members_from_statement(statement, members, variables, constants, types);
            }
        }

        IRStatement::If(if_node) => {
            collect_members_from_expr(&if_node.condition, members, variables, constants, types);

            for statement in &if_node.then_branch {
                collect_members_from_statement(statement, members, variables, constants, types);
            }

            for statement in &if_node.else_branch {
                collect_members_from_statement(statement, members, variables, constants, types);
            }
        }

        IRStatement::Match(match_node) => {
            collect_members_from_expr(&match_node.scrutinee, members, variables, constants, types);

            for arm in &match_node.arms {
                for statement in &arm.body {
                    collect_members_from_statement(statement, members, variables, constants, types);
                }
            }

            if let Some(otherwise) = &match_node.otherwise {
                for statement in otherwise {
                    collect_members_from_statement(statement, members, variables, constants, types);
                }
            }
        }

        IRStatement::Try(try_node) => {
            for statement in &try_node.try_body {
                collect_members_from_statement(statement, members, variables, constants, types);
            }

            for statement in &try_node.catch_body {
                collect_members_from_statement(statement, members, variables, constants, types);
            }

            for statement in &try_node.finally_body {
                collect_members_from_statement(statement, members, variables, constants, types);
            }
        }

        IRStatement::Move(move_node) => {
            collect_members_from_expr(&move_node.source, members, variables, constants, types);
            collect_members_from_expr(&move_node.target, members, variables, constants, types);
        }

        IRStatement::Clone(clone_node) => {
            collect_members_from_expr(&clone_node.source, members, variables, constants, types);
            collect_members_from_expr(&clone_node.target, members, variables, constants, types);
        }
    }
}

pub fn tooling_analysis_from_ir(ir_root: &IRRoot) -> Result<ToolingAnalysis, String> {
    let analyzer = analyze_ir_with_symbols(ir_root).map_err(|error| error.message())?;

    let mut tooling = analyzer.tooling;

    for type_decl in &ir_root.types {
        if let IRTypeDecl::Struct(struct_decl) = type_decl {
            let fields = struct_decl
                .fields
                .iter()
                .map(|field| (field.name.clone(), field.ty.clone()))
                .collect();

            tooling.types.insert(struct_decl.name.clone(), fields);
        }
    }

    for function in &ir_root.functions {
        if let Some(function_tooling) = tooling.functions.get_mut(&function.name) {
            for statement in &function.body {
                collect_members_from_statement(
                    statement,
                    &mut function_tooling.members,
                    &function_tooling.variables,
                    &function_tooling.constants,
                    &tooling.types,
                );
            }
        }
    }

    Ok(tooling)
}
