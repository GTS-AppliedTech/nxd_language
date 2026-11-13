use crate::diagnostics::severity::DiagnosticSeverity;
use crate::diagnostics::span::SourceSpan;
use crate::diagnostics::types::WarningDiagnostic;
use crate::frontend::ast::nodes::*;

use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DeclarationKind {
    Variable,
    Constant,
}

#[derive(Clone, Debug)]
struct DeclarationInfo {
    span: SourceSpan,
    kind: DeclarationKind,
}

fn collect_called_functions_from_expression(expression: &AstExpr, called: &mut HashSet<String>) {
    match expression {
        AstExpr::Call(call) => {
            called.insert(call.func.clone());

            for argument in &call.args {
                collect_called_functions_from_expression(argument, called);
            }
        }

        AstExpr::Pipeline(pipeline) => {
            called.insert(pipeline.func.clone());
            collect_called_functions_from_expression(&pipeline.value, called);
        }

        AstExpr::Binary(binary) => {
            collect_called_functions_from_expression(&binary.left, called);
            collect_called_functions_from_expression(&binary.right, called);
        }

        AstExpr::Unary(unary) => {
            collect_called_functions_from_expression(&unary.expr, called);
        }

        AstExpr::ObjectInit(object) => {
            for value in object.fields.values() {
                collect_called_functions_from_expression(value, called);
            }
        }

        AstExpr::MemberAccess(access) => {
            collect_called_functions_from_expression(&access.object, called);
        }

        AstExpr::Literal(literal) => match literal {
            AstLiteral::List(items) => {
                for item in items {
                    collect_called_functions_from_expression(item, called);
                }
            }

            AstLiteral::Map(entries) => {
                for value in entries.values() {
                    collect_called_functions_from_expression(value, called);
                }
            }

            AstLiteral::Lambda { body, .. } => {
                collect_called_functions_from_expression(body, called);
            }

            _ => {}
        },

        AstExpr::Var(_) => {}
    }
}

fn collect_called_functions_from_statement(statement: &AstStatement, called: &mut HashSet<String>) {
    match statement {
        AstStatement::Let(let_statement) => {
            collect_called_functions_from_expression(&let_statement.value, called);
        }

        AstStatement::Const(const_statement) => {
            collect_called_functions_from_expression(&const_statement.value, called);
        }

        AstStatement::Return(return_statement) => {
            collect_called_functions_from_expression(&return_statement.value, called);
        }

        AstStatement::If(if_statement) => {
            collect_called_functions_from_expression(&if_statement.condition, called);

            for statement in &if_statement.then_branch {
                collect_called_functions_from_statement(statement, called);
            }

            for statement in &if_statement.else_branch {
                collect_called_functions_from_statement(statement, called);
            }
        }

        AstStatement::Loop(loop_statement) => {
            for statement in &loop_statement.body {
                collect_called_functions_from_statement(statement, called);
            }
        }

        AstStatement::Try(try_statement) => {
            for statement in &try_statement.try_body {
                collect_called_functions_from_statement(statement, called);
            }

            for statement in &try_statement.catch_body {
                collect_called_functions_from_statement(statement, called);
            }

            for statement in &try_statement.finally_body {
                collect_called_functions_from_statement(statement, called);
            }
        }

        AstStatement::Match(match_statement) => {
            collect_called_functions_from_expression(&match_statement.scrutinee, called);

            for arm in &match_statement.arms {
                for statement in &arm.body {
                    collect_called_functions_from_statement(statement, called);
                }
            }

            if let Some(otherwise) = &match_statement.otherwise {
                for statement in otherwise {
                    collect_called_functions_from_statement(statement, called);
                }
            }
        }

        AstStatement::Move(move_statement) => {
            collect_called_functions_from_expression(&move_statement.source, called);
            collect_called_functions_from_expression(&move_statement.target, called);
        }

        AstStatement::Clone(clone_statement) => {
            collect_called_functions_from_expression(&clone_statement.source, called);
            collect_called_functions_from_expression(&clone_statement.target, called);
        }

        AstStatement::Expr(expression) => {
            collect_called_functions_from_expression(expression, called);
        }
    }
}

fn collect_usage(
    module: &AstModule,
    declared: &mut HashMap<String, DeclarationInfo>,
    used: &mut HashSet<String>,
) {
    for node in &module.body {
        collect_node_usage(node, declared, used);
    }
}

fn collect_node_usage(
    node: &AstNode,
    declared: &mut HashMap<String, DeclarationInfo>,
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
    declared: &mut HashMap<String, DeclarationInfo>,
    used: &mut HashSet<String>,
) {
    match statement {
        AstStatement::Let(let_statement) => {
            declared.insert(
                let_statement.name.clone(),
                DeclarationInfo {
                    span: SourceSpan {
                        line: let_statement.line,
                        column: let_statement.column,
                        end_line: let_statement.line,
                        end_column: let_statement.column + let_statement.name.len() as u32,
                    },
                    kind: DeclarationKind::Variable,
                },
            );

            collect_expression_usage(&let_statement.value, declared, used);
        }

        AstStatement::Const(const_statement) => {
            declared.insert(
                const_statement.name.clone(),
                DeclarationInfo {
                    span: SourceSpan {
                        line: const_statement.line,
                        column: const_statement.column,
                        end_line: const_statement.line,
                        end_column: const_statement.column + const_statement.name.len() as u32,
                    },
                    kind: DeclarationKind::Constant,
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
    declared: &mut HashMap<String, DeclarationInfo>,
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

        AstExpr::MemberAccess(access) => {
            collect_expression_usage(&access.object, declared, used);
        }
        AstExpr::Pipeline(pipeline) => {
            collect_expression_usage(&pipeline.value, declared, used);
        }
    }
}

fn collect_literal_usage(
    literal: &AstLiteral,
    declared: &mut HashMap<String, DeclarationInfo>,
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

    for (name, declaration) in declared.iter() {
        if used.contains(name) {
            continue;
        }

        match declaration.kind {
            DeclarationKind::Variable => {
                warnings.push(WarningDiagnostic::new(
                    "NXD-W2001",
                    format!("NXD-W2001: Variable '{}' declared but never used.", name),
                    declaration.span.clone(),
                ));
            }

            DeclarationKind::Constant => {
                warnings.push(WarningDiagnostic::new(
                    "NXD-W2010",
                    format!("NXD-W2010: CONST '{}' declared but never used.", name),
                    declaration.span.clone(),
                ));
            }
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

pub fn find_unused_parameters(module: &AstModule) -> Vec<WarningDiagnostic> {
    let mut warnings = Vec::new();

    for node in &module.body {
        match node {
            AstNode::Function(function) => {
                collect_unused_parameters_for_function(function, &mut warnings);
            }

            AstNode::Impl(implementation) => {
                for method in &implementation.methods {
                    collect_unused_parameters_for_function(method, &mut warnings);
                }
            }

            _ => {}
        }
    }

    warnings
}

fn collect_unused_parameters_for_function(
    function: &AstFunction,
    warnings: &mut Vec<WarningDiagnostic>,
) {
    let mut declared = HashMap::new();
    let mut used = HashSet::new();

    for statement in &function.body {
        collect_statement_usage(statement, &mut declared, &mut used);
    }

    for parameter in &function.params {
        if !used.contains(&parameter.name) {
            warnings.push(WarningDiagnostic::new(
                "NXD-W2005",
                format!(
                    "NXD-W2005: Parameter '{}' declared but never used.",
                    parameter.name
                ),
                parameter.span.clone(),
            ));
        }
    }
}

pub fn find_unused_functions(module: &AstModule) -> Vec<WarningDiagnostic> {
    let mut called = HashSet::new();

    // First collect every function call made anywhere in module functions.
    for node in &module.body {
        if let AstNode::Function(function) = node {
            for statement in &function.body {
                collect_called_functions_from_statement(statement, &mut called);
            }
        }
    }

    let mut warnings = Vec::new();

    // Then find declared functions that were never called.
    for node in &module.body {
        if let AstNode::Function(function) = node {
            // MAIN is the program entry point, not something expected
            // to be called explicitly.
            if function.name.eq_ignore_ascii_case("MAIN") {
                continue;
            }

            if !called.contains(&function.name) {
                warnings.push(WarningDiagnostic::new(
                    "NXD-W2006",
                    format!(
                        "NXD-W2006: FUNC '{}' declared but never used.",
                        function.name
                    ),
                    function.name_span.clone(),
                ));
            }
        }
    }

    warnings
}

fn check_constant_condition(if_statement: &AstIf, warnings: &mut Vec<WarningDiagnostic>) {
    let AstExpr::Literal(AstLiteral::Bool(value)) = &if_statement.condition else {
        return;
    };

    let condition_text = if *value { "true" } else { "false" };
    let condition_column = if_statement.column + 3;

    warnings.push(WarningDiagnostic::new(
        "NXD-W2007",
        format!("NXD-W2007: Condition is always {}.", condition_text),
        SourceSpan {
            line: if_statement.line,
            column: condition_column,
            end_line: if_statement.line,
            end_column: condition_column + condition_text.len() as u32,
        },
    ));
}

fn visit_statements_for_constant_conditions(
    statements: &[AstStatement],
    warnings: &mut Vec<WarningDiagnostic>,
) {
    for statement in statements {
        match statement {
            AstStatement::If(if_statement) => {
                check_constant_condition(if_statement, warnings);

                visit_statements_for_constant_conditions(&if_statement.then_branch, warnings);

                visit_statements_for_constant_conditions(&if_statement.else_branch, warnings);
            }

            AstStatement::Loop(loop_statement) => {
                visit_statements_for_constant_conditions(&loop_statement.body, warnings);
            }

            AstStatement::Try(try_statement) => {
                visit_statements_for_constant_conditions(&try_statement.try_body, warnings);

                visit_statements_for_constant_conditions(&try_statement.catch_body, warnings);

                visit_statements_for_constant_conditions(&try_statement.finally_body, warnings);
            }

            AstStatement::Match(match_statement) => {
                for arm in &match_statement.arms {
                    visit_statements_for_constant_conditions(&arm.body, warnings);
                }

                if let Some(otherwise) = &match_statement.otherwise {
                    visit_statements_for_constant_conditions(otherwise, warnings);
                }
            }

            _ => {}
        }
    }
}

pub fn find_constant_conditions(module: &AstModule) -> Vec<WarningDiagnostic> {
    let mut warnings = Vec::new();

    for node in &module.body {
        match node {
            AstNode::Function(function) => {
                visit_statements_for_constant_conditions(&function.body, &mut warnings);
            }

            AstNode::Impl(implementation) => {
                for method in &implementation.methods {
                    visit_statements_for_constant_conditions(&method.body, &mut warnings);
                }
            }

            AstNode::Statement(statement) => {
                visit_statements_for_constant_conditions(
                    std::slice::from_ref(statement),
                    &mut warnings,
                );
            }

            _ => {}
        }
    }

    warnings
}

fn check_redundant_match_arms(match_statement: &AstMatch, warnings: &mut Vec<WarningDiagnostic>) {
    let mut seen_patterns = HashSet::new();

    for arm in &match_statement.arms {
        if !seen_patterns.insert(arm.pattern.clone()) {
            warnings.push(WarningDiagnostic::new(
                "NXD-W2008",
                format!(
                    "NXD-W2008: Duplicate/redundant MATCH arm '{}'.",
                    arm.pattern
                ),
                arm.pattern_span.clone(),
            ));
        }
    }
}

fn visit_statements_for_redundant_match_arms(
    statements: &[AstStatement],
    warnings: &mut Vec<WarningDiagnostic>,
) {
    for statement in statements {
        match statement {
            AstStatement::If(if_statement) => {
                visit_statements_for_redundant_match_arms(&if_statement.then_branch, warnings);

                visit_statements_for_redundant_match_arms(&if_statement.else_branch, warnings);
            }

            AstStatement::Loop(loop_statement) => {
                visit_statements_for_redundant_match_arms(&loop_statement.body, warnings);
            }

            AstStatement::Try(try_statement) => {
                visit_statements_for_redundant_match_arms(&try_statement.try_body, warnings);

                visit_statements_for_redundant_match_arms(&try_statement.catch_body, warnings);

                visit_statements_for_redundant_match_arms(&try_statement.finally_body, warnings);
            }

            AstStatement::Match(match_statement) => {
                check_redundant_match_arms(match_statement, warnings);

                for arm in &match_statement.arms {
                    visit_statements_for_redundant_match_arms(&arm.body, warnings);
                }

                if let Some(otherwise) = &match_statement.otherwise {
                    visit_statements_for_redundant_match_arms(otherwise, warnings);
                }
            }

            _ => {}
        }
    }
}

pub fn find_redundant_match_arms(module: &AstModule) -> Vec<WarningDiagnostic> {
    let mut warnings = Vec::new();

    for node in &module.body {
        match node {
            AstNode::Function(function) => {
                visit_statements_for_redundant_match_arms(&function.body, &mut warnings);
            }

            AstNode::Impl(implementation) => {
                for method in &implementation.methods {
                    visit_statements_for_redundant_match_arms(&method.body, &mut warnings);
                }
            }

            AstNode::Statement(statement) => {
                visit_statements_for_redundant_match_arms(
                    std::slice::from_ref(statement),
                    &mut warnings,
                );
            }

            _ => {}
        }
    }

    warnings
}

fn check_non_exhaustive_match(match_statement: &AstMatch, warnings: &mut Vec<WarningDiagnostic>) {
    if match_statement.otherwise.is_some() {
        return;
    }

    warnings.push(WarningDiagnostic::new(
        "NXD-W2009",
        "NXD-W2009: Non-exhaustive MATCH.".to_string(),
        match_statement.span.clone(),
    ));
}

fn visit_statements_for_non_exhaustive_matches(
    statements: &[AstStatement],
    warnings: &mut Vec<WarningDiagnostic>,
) {
    for statement in statements {
        match statement {
            AstStatement::If(if_statement) => {
                visit_statements_for_non_exhaustive_matches(&if_statement.then_branch, warnings);

                visit_statements_for_non_exhaustive_matches(&if_statement.else_branch, warnings);
            }

            AstStatement::Loop(loop_statement) => {
                visit_statements_for_non_exhaustive_matches(&loop_statement.body, warnings);
            }

            AstStatement::Try(try_statement) => {
                visit_statements_for_non_exhaustive_matches(&try_statement.try_body, warnings);

                visit_statements_for_non_exhaustive_matches(&try_statement.catch_body, warnings);

                visit_statements_for_non_exhaustive_matches(&try_statement.finally_body, warnings);
            }

            AstStatement::Match(match_statement) => {
                check_non_exhaustive_match(match_statement, warnings);

                for arm in &match_statement.arms {
                    visit_statements_for_non_exhaustive_matches(&arm.body, warnings);
                }

                if let Some(otherwise) = &match_statement.otherwise {
                    visit_statements_for_non_exhaustive_matches(otherwise, warnings);
                }
            }

            _ => {}
        }
    }
}

pub fn find_non_exhaustive_matches(module: &AstModule) -> Vec<WarningDiagnostic> {
    let mut warnings = Vec::new();

    for node in &module.body {
        match node {
            AstNode::Function(function) => {
                visit_statements_for_non_exhaustive_matches(&function.body, &mut warnings);
            }

            AstNode::Impl(implementation) => {
                for method in &implementation.methods {
                    visit_statements_for_non_exhaustive_matches(&method.body, &mut warnings);
                }
            }

            AstNode::Statement(statement) => {
                visit_statements_for_non_exhaustive_matches(
                    std::slice::from_ref(statement),
                    &mut warnings,
                );
            }

            _ => {}
        }
    }

    warnings
}
