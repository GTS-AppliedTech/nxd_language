use crate::diagnostics::parser::parse_with_diagnostics;
use crate::diagnostics::severity::DiagnosticSeverity;
use crate::diagnostics::span::SourceSpan;
use crate::diagnostics::types::{ParserError, WarningDiagnostic};
use crate::diagnostics::warnings::{
    find_constant_conditions, find_non_exhaustive_matches, find_redundant_match_arms,
    find_shadowed_bindings, find_unreachable_code, find_unused_functions, find_unused_parameters,
    find_unused_variables,
};
use crate::frontend::ast::nodes::{AstFunction, AstModule, AstNode};
use crate::ir::lowering::{lower_function, lower_module, lower_types};
use crate::ir::nodes::IRRoot;
use crate::orchestrator::semantic_diagnostics_from_ir;

#[derive(Clone, Debug)]
pub struct LanguageDiagnostic {
    pub code: String,
    pub message: String,
    pub span: SourceSpan,
    pub severity: DiagnosticSeverity,
}

#[derive(Clone, Debug)]
pub struct ValidationResult {
    pub diagnostics: Vec<LanguageDiagnostic>,
}

fn parser_error_to_diagnostic(error: &ParserError) -> LanguageDiagnostic {
    LanguageDiagnostic {
        code: error.code.clone(),
        message: error.message.clone(),
        span: error.span.clone(),
        severity: error.severity.clone(),
    }
}

fn warning_to_diagnostic(warning: &WarningDiagnostic) -> LanguageDiagnostic {
    LanguageDiagnostic {
        code: warning.code.clone(),
        message: warning.message.clone(),
        span: warning.span.clone(),
        severity: warning.severity.clone(),
    }
}

fn build_ir(module: &AstModule, ast_types: &[AstNode], ast_functions: &[AstFunction]) -> IRRoot {
    IRRoot {
        module: lower_module(module),
        types: lower_types(ast_types),
        functions: ast_functions.iter().map(lower_function).collect(),
        traits: Vec::new(),
        impls: Vec::new(),
        statements: Vec::new(),
    }
}

pub fn validate_source(source: &str) -> ValidationResult {
    let parse_result = parse_with_diagnostics(source);

    let (module, ast_types, ast_functions, parser_errors) = match parse_result {
        Ok(result) => result,

        Err(error) => {
            return ValidationResult {
                diagnostics: vec![parser_error_to_diagnostic(&error)],
            };
        }
    };

    let mut diagnostics: Vec<LanguageDiagnostic> = parser_errors
        .iter()
        .map(parser_error_to_diagnostic)
        .collect();

    diagnostics.extend(
        find_unused_variables(&module)
            .iter()
            .map(warning_to_diagnostic),
    );

    diagnostics.extend(
        find_unreachable_code(&module)
            .iter()
            .map(warning_to_diagnostic),
    );

    diagnostics.extend(
        find_shadowed_bindings(&module)
            .iter()
            .map(warning_to_diagnostic),
    );

    diagnostics.extend(
        find_unused_parameters(&module)
            .iter()
            .map(warning_to_diagnostic),
    );

    diagnostics.extend(
        find_unused_functions(&module)
            .iter()
            .map(warning_to_diagnostic),
    );

    diagnostics.extend(
        find_constant_conditions(&module)
            .iter()
            .map(warning_to_diagnostic),
    );

    diagnostics.extend(
        find_redundant_match_arms(&module)
            .iter()
            .map(warning_to_diagnostic),
    );

    diagnostics.extend(
        find_non_exhaustive_matches(&module)
            .iter()
            .map(warning_to_diagnostic),
    );

    // Preserve the existing diagnostic precedence:
    // semantic diagnostics only run when parsing produced no errors.
    if parser_errors.is_empty() {
        let ir_root = build_ir(&module, &ast_types, &ast_functions);

        let semantic_response = semantic_diagnostics_from_ir(&ir_root);

        diagnostics.extend(semantic_response.diagnostics.into_iter().map(|diagnostic| {
            LanguageDiagnostic {
                code: diagnostic.code,
                message: diagnostic.message,
                span: SourceSpan {
                    line: diagnostic.line as u32,
                    column: diagnostic.column as u32,
                    end_line: diagnostic.end_line as u32,
                    end_column: diagnostic.end_column as u32,
                },
                severity: if diagnostic.severity == "warning" {
                    DiagnosticSeverity::Warning
                } else {
                    DiagnosticSeverity::Error
                },
            }
        }));
    }

    ValidationResult { diagnostics }
}
