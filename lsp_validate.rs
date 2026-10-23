use serde::Serialize;

use std::io::{self, Read};

use nxd_language::diagnostics::parser::parse_with_diagnostics;

use nxd_language::diagnostics::severity::DiagnosticSeverity;

use nxd_language::diagnostics::types::{ParserError, WarningDiagnostic};

use nxd_language::diagnostics::warnings::{
    find_shadowed_bindings, find_unreachable_code, find_unused_variables,
};

use nxd_language::frontend::ast::nodes::{AstFunction, AstModule, AstNode};

use nxd_language::ir::lowering::{lower_function, lower_module, lower_types};

use nxd_language::ir::nodes::IRRoot;

use nxd_language::orchestrator::semantic_diagnostics_from_ir;

#[derive(Debug, Serialize)]
struct LspDiagnostic {
    code: String,
    message: String,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    severity: String,
}

#[derive(Debug, Serialize)]
struct ValidationResponse {
    ok: bool,
    diagnostics: Vec<LspDiagnostic>,
}

fn severity_name(severity: &DiagnosticSeverity) -> String {
    match severity {
        DiagnosticSeverity::Error => "error".to_string(),

        DiagnosticSeverity::Warning => "warning".to_string(),
    }
}

fn parser_error_to_lsp(error: &ParserError) -> LspDiagnostic {
    LspDiagnostic {
        code: error.code.clone(),
        message: error.message.clone(),

        line: error.span.line as usize,
        column: error.span.column as usize,

        end_line: error.span.end_line as usize,
        end_column: error.span.end_column as usize,

        severity: severity_name(&error.severity),
    }
}

fn warning_to_lsp(warning: &WarningDiagnostic) -> LspDiagnostic {
    LspDiagnostic {
        code: warning.code.clone(),
        message: warning.message.clone(),

        line: warning.span.line as usize,
        column: warning.span.column as usize,

        end_line: warning.span.end_line as usize,
        end_column: warning.span.end_column as usize,

        severity: severity_name(&warning.severity),
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

fn validate_source(source: &str) -> ValidationResponse {
    let parse_result = parse_with_diagnostics(source);

    let (module, ast_types, ast_functions, parser_errors) = match parse_result {
        Ok(result) => result,

        Err(error) => {
            return ValidationResponse {
                ok: false,

                diagnostics: vec![parser_error_to_lsp(&error)],
            };
        }
    };

    let mut diagnostics: Vec<LspDiagnostic> =
        parser_errors.iter().map(parser_error_to_lsp).collect();

    let unused_warnings = find_unused_variables(&module);

    diagnostics.extend(unused_warnings.iter().map(warning_to_lsp));

    let unreachable_warnings = find_unreachable_code(&module);

    diagnostics.extend(unreachable_warnings.iter().map(warning_to_lsp));

    let shadowed_warnings = find_shadowed_bindings(&module);

    diagnostics.extend(shadowed_warnings.iter().map(warning_to_lsp));

    /*
     * Preserve the Python behavior:
     *
     * semantic diagnostics only run when there
     * are no collected parser errors.
     */
    if parser_errors.is_empty() {
        let ir_root = build_ir(&module, &ast_types, &ast_functions);

        let semantic_response = semantic_diagnostics_from_ir(&ir_root);

        diagnostics.extend(semantic_response.diagnostics.into_iter().map(|diagnostic| {
            LspDiagnostic {
                code: diagnostic.code,

                message: diagnostic.message,

                line: diagnostic.line,

                column: diagnostic.column,

                end_line: diagnostic.end_line,

                end_column: diagnostic.end_column,

                severity: diagnostic.severity,
            }
        }));
    }

    ValidationResponse {
        ok: true,
        diagnostics,
    }
}

fn print_response(response: &ValidationResponse) {
    match serde_json::to_string(response) {
        Ok(json) => {
            println!("{}", json);
        }

        Err(error) => {
            /*
             * Keep stdout valid JSON even if response
             * serialization itself unexpectedly fails.
             */
            let fallback = serde_json::json!({
                "ok": false,

                "diagnostics": [
                    {
                        "code":
                            "NXD-LSP0003",

                        "message":
                            format!(
                                "NXD-LSP0003: Failed to serialize diagnostic response: {}",
                                error,
                            ),

                        "line": 1,
                        "column": 1,
                        "end_line": 1,
                        "end_column": 2,
                        "severity": "error",
                    }
                ]
            });

            println!("{}", fallback);
        }
    }
}

fn main() {
    let mut source = String::new();

    if let Err(error) = io::stdin().read_to_string(&mut source) {
        let response = ValidationResponse {
            ok: false,

            diagnostics: vec![LspDiagnostic {
                code: "NXD-LSP0002".to_string(),

                message: format!(
                    "NXD-LSP0002: Failed to read NXD source from stdin: {}",
                    error,
                ),

                line: 1,
                column: 1,
                end_line: 1,
                end_column: 2,

                severity: "error".to_string(),
            }],
        };

        print_response(&response);

        return;
    }

    let response = validate_source(&source);

    print_response(&response);
}
