use serde::Serialize;

use std::io::{self, Read};

use nxd_language::diagnostics::severity::DiagnosticSeverity;
use nxd_language::diagnostics::validation::{LanguageDiagnostic, validate_source};

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

fn language_diagnostic_to_lsp(diagnostic: &LanguageDiagnostic) -> LspDiagnostic {
    LspDiagnostic {
        code: diagnostic.code.clone(),
        message: diagnostic.message.clone(),

        line: diagnostic.span.line as usize,
        column: diagnostic.span.column as usize,

        end_line: diagnostic.span.end_line as usize,
        end_column: diagnostic.span.end_column as usize,

        severity: severity_name(&diagnostic.severity),
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
                        "code": "NXD-LSP0003",

                        "message": format!(
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

    let validation = validate_source(&source);

    let response = ValidationResponse {
        ok: true,

        diagnostics: validation
            .diagnostics
            .iter()
            .map(language_diagnostic_to_lsp)
            .collect(),
    };

    print_response(&response);
}
