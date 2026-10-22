use crate::diagnostics::severity::DiagnosticSeverity;
use crate::diagnostics::span::SourceSpan;
use crate::diagnostics::types::WarningDiagnostic;
use crate::frontend::ast::nodes::*;

use std::collections::{HashMap, HashSet};

pub fn collect_usage(
    module: &AstModule,
    declared: &mut HashMap<String, SourceSpan>,
    used: &mut HashSet<String>,
) {
    for node in &module.body {
        collect_node_usage(node, declared, used);
    }
}

fn collect_node_usage(
    node: &AstNode,
    declared: &mut HashMap<String, SourceSpan>,
    used: &mut HashSet<String>,
) {
    match node {
        AstNode::Function(function) => {
            for statement in &function.body {
                collect_statement_usage(statement, declared, used);
            }
        }

        AstNode::Statement(statement) => {
            collect_statement_usage(statement, declared, used);
        }

        AstNode::Impl(implementation) => {
            for method in &implementation.methods {
                for statement in &method.body {
                    collect_statement_usage(statement, declared, used);
                }
            }
        }

        AstNode::Struct(_) | AstNode::Enum(_) | AstNode::Union(_) | AstNode::Trait(_) => {}
    }
}

fn collect_statement_usage(
    statement: &AstStatement,
    declared: &mut HashMap<String, SourceSpan>,
    used: &mut HashSet<String>,
) {
    match statement {
        AstStatement::Let(let_statement) => {
            declared.insert(
                let_statement.name.clone(),
                SourceSpan {
                    line: let_statement.line,
                    column: let_statement.column,
                    end_line: let_statement.line,
                    end_column: let_statement.column + let_statement.name.len() as u32,
                },
            );

            collect_expression_usage(&let_statement.value, declared, used);
        }

        AstStatement::Const(const_statement) => {
            declared.insert(
                const_statement.name.clone(),
                SourceSpan {
                    line: const_statement.line,
                    column: const_statement.column,
                    end_line: const_statement.line,
                    end_column: const_statement.column + const_statement.name.len() as u32,
                },
            );

            collect_expression_usage(&const_statement.value, declared, used);
        }

        AstStatement::Return(return_statement) => {
            collect_expression_usage(&return_statement.value, declared, used);
        }

        AstStatement::If(if_statement) => {
            collect_expression_usage(&if_statement.condition, declared, used);

            for statement in &if_statement.then_branch {
                collect_statement_usage(statement, declared, used);
            }

            for statement in &if_statement.else_branch {
                collect_statement_usage(statement, declared, used);
            }
        }

        AstStatement::Loop(loop_statement) => {
            for statement in &loop_statement.body {
                collect_statement_usage(statement, declared, used);
            }
        }

        AstStatement::Try(try_statement) => {
            for statement in &try_statement.try_body {
                collect_statement_usage(statement, declared, used);
            }

            for statement in &try_statement.catch_body {
                collect_statement_usage(statement, declared, used);
            }

            for statement in &try_statement.finally_body {
                collect_statement_usage(statement, declared, used);
            }
        }

        AstStatement::Match(match_statement) => {
            collect_expression_usage(&match_statement.scrutinee, declared, used);

            for arm in &match_statement.arms {
                for statement in &arm.body {
                    collect_statement_usage(statement, declared, used);
                }
            }

            if let Some(otherwise) = &match_statement.otherwise {
                for statement in otherwise {
                    collect_statement_usage(statement, declared, used);
                }
            }
        }

        AstStatement::Move(move_statement) => {
            collect_expression_usage(&move_statement.source, declared, used);

            collect_expression_usage(&move_statement.target, declared, used);
        }

        AstStatement::Clone(clone_statement) => {
            collect_expression_usage(&clone_statement.source, declared, used);

            collect_expression_usage(&clone_statement.target, declared, used);
        }

        AstStatement::Expr(expression) => {
            collect_expression_usage(expression, declared, used);
        }
    }
}

fn collect_expression_usage(
    expression: &AstExpr,
    declared: &mut HashMap<String, SourceSpan>,
    used: &mut HashSet<String>,
) {
    match expression {
        AstExpr::Var(variable) => {
            used.insert(variable.name.clone());
        }

        AstExpr::Binary(binary) => {
            collect_expression_usage(&binary.left, declared, used);

            collect_expression_usage(&binary.right, declared, used);
        }

        AstExpr::Unary(unary) => {
            collect_expression_usage(&unary.expr, declared, used);
        }

        AstExpr::Call(call) => {
            for argument in &call.args {
                collect_expression_usage(argument, declared, used);
            }
        }

        AstExpr::ObjectInit(object) => {
            for value in object.fields.values() {
                collect_expression_usage(value, declared, used);
            }
        }

        AstExpr::Literal(literal) => {
            collect_literal_usage(literal, declared, used);
        }

        AstExpr::Pipeline(_) => {
            // Fill this in once we compare the AstPipeline fields.
        }
    }
}

fn collect_literal_usage(
    literal: &AstLiteral,
    declared: &mut HashMap<String, SourceSpan>,
    used: &mut HashSet<String>,
) {
    match literal {
        AstLiteral::List(items) => {
            for item in items {
                collect_expression_usage(item, declared, used);
            }
        }

        AstLiteral::Map(entries) => {
            for value in entries.values() {
                collect_expression_usage(value, declared, used);
            }
        }

        AstLiteral::Lambda { params: _, body } => {
            collect_expression_usage(body, declared, used);
        }

        AstLiteral::Int(_)
        | AstLiteral::Float(_)
        | AstLiteral::String(_)
        | AstLiteral::Bool(_)
        | AstLiteral::None => {}
    }
}

pub fn find_unused_variables(module: &AstModule) -> Vec<WarningDiagnostic> {
    let mut declared = HashMap::new();
    let mut used = HashSet::new();

    collect_usage(module, &mut declared, &mut used);

    let mut warnings = Vec::new();

    for (name, span) in declared.iter() {
        if !used.contains(name) {
            warnings.push(WarningDiagnostic {
                code: "NXD-W2001".to_string(),

                message: format!("NXD-W2001: Variable '{}' declared but never used.", name,),

                span: span.clone(),

                severity: DiagnosticSeverity::Warning,
            });
        }
    }

    warnings
}

pub fn find_unreachable_code(module: &AstModule) -> Vec<WarningDiagnostic> {
    let mut warnings = Vec::new();

    for item in &module.body {
        if let AstNode::Function(function) = item {
            let mut found_return = false;

            for statement in &function.body {
                if found_return {
                    warnings.push(WarningDiagnostic {
                        code: "NXD-W2002".to_string(),

                        message: "NXD-W2002: Unreachable code.".to_string(),

                        span: statement.span(),

                        severity: DiagnosticSeverity::Warning,
                    });
                }

                if matches!(statement, AstStatement::Return(_)) {
                    found_return = true;
                }
            }
        }
    }

    warnings
}

pub fn find_shadowed_bindings(module: &AstModule) -> Vec<WarningDiagnostic> {
    let mut warnings = Vec::new();
    let outer_names = HashSet::new();

    visit_nodes_for_shadowing(&module.body, &outer_names, &mut warnings);

    warnings
}

fn visit_nodes_for_shadowing(
    nodes: &[AstNode],
    outer_names: &HashSet<String>,
    warnings: &mut Vec<WarningDiagnostic>,
) {
    let mut current_names = HashSet::new();

    for node in nodes {
        match node {
            AstNode::Statement(statement) => {
                check_statement_declaration(statement, outer_names, &mut current_names, warnings);

                let visible_names = combined_names(outer_names, &current_names);

                visit_nested_statement_scopes(statement, &visible_names, warnings);
            }

            AstNode::Function(function) => {
                let visible_names = combined_names(outer_names, &current_names);

                visit_statements_for_shadowing(&function.body, &visible_names, warnings);
            }

            AstNode::Impl(implementation) => {
                let visible_names = combined_names(outer_names, &current_names);

                for method in &implementation.methods {
                    visit_statements_for_shadowing(&method.body, &visible_names, warnings);
                }
            }

            AstNode::Struct(_) | AstNode::Enum(_) | AstNode::Union(_) | AstNode::Trait(_) => {}
        }
    }
}

fn visit_statements_for_shadowing(
    statements: &[AstStatement],
    outer_names: &HashSet<String>,
    warnings: &mut Vec<WarningDiagnostic>,
) {
    let mut current_names = HashSet::new();

    for statement in statements {
        check_statement_declaration(statement, outer_names, &mut current_names, warnings);

        let visible_names = combined_names(outer_names, &current_names);

        visit_nested_statement_scopes(statement, &visible_names, warnings);
    }
}

fn check_statement_declaration(
    statement: &AstStatement,
    outer_names: &HashSet<String>,
    current_names: &mut HashSet<String>,
    warnings: &mut Vec<WarningDiagnostic>,
) {
    let declaration = match statement {
        AstStatement::Let(let_statement) => Some((
            &let_statement.name,
            let_statement.line,
            let_statement.column,
        )),

        AstStatement::Const(const_statement) => Some((
            &const_statement.name,
            const_statement.line,
            const_statement.column,
        )),

        _ => None,
    };

    let Some((name, line, column)) = declaration else {
        return;
    };

    if current_names.contains(name) || outer_names.contains(name) {
        warnings.push(WarningDiagnostic {
            code: "NXD-W2003".to_string(),

            message: format!("NXD-W2003: Binding '{}' shadows a previous binding.", name,),

            span: SourceSpan {
                line,
                column,
                end_line: line,
                end_column: column + name.len() as u32,
            },

            severity: DiagnosticSeverity::Warning,
        });
    }

    current_names.insert(name.clone());
}

fn visit_nested_statement_scopes(
    statement: &AstStatement,
    visible_names: &HashSet<String>,
    warnings: &mut Vec<WarningDiagnostic>,
) {
    match statement {
        AstStatement::If(if_statement) => {
            visit_statements_for_shadowing(&if_statement.then_branch, visible_names, warnings);

            visit_statements_for_shadowing(&if_statement.else_branch, visible_names, warnings);
        }

        AstStatement::Loop(loop_statement) => {
            visit_statements_for_shadowing(&loop_statement.body, visible_names, warnings);
        }

        AstStatement::Try(try_statement) => {
            visit_statements_for_shadowing(&try_statement.try_body, visible_names, warnings);

            visit_statements_for_shadowing(&try_statement.catch_body, visible_names, warnings);

            visit_statements_for_shadowing(&try_statement.finally_body, visible_names, warnings);
        }

        AstStatement::Match(match_statement) => {
            for arm in &match_statement.arms {
                visit_statements_for_shadowing(&arm.body, visible_names, warnings);
            }

            if let Some(otherwise) = &match_statement.otherwise {
                visit_statements_for_shadowing(otherwise, visible_names, warnings);
            }
        }

        AstStatement::Move(_)
        | AstStatement::Clone(_)
        | AstStatement::Let(_)
        | AstStatement::Const(_)
        | AstStatement::Return(_)
        | AstStatement::Expr(_) => {}
    }
}

fn combined_names(
    outer_names: &HashSet<String>,
    current_names: &HashSet<String>,
) -> HashSet<String> {
    outer_names.union(current_names).cloned().collect()
}

pub fn find_unused_imports(_module: &AstModule) -> Vec<WarningDiagnostic> {
    let warnings = Vec::new();

    warnings
}

impl WarningDiagnostic {
    pub fn new(code: impl Into<String>, message: impl Into<String>, span: SourceSpan) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            span,
            severity: DiagnosticSeverity::Warning,
        }
    }
}
