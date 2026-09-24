use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SemanticDiagnostic {
    pub code: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
    pub severity: String,
}

#[derive(Debug, Serialize)]
pub struct SemanticDiagnosticResponse {
    pub ok: bool,
    pub diagnostics: Vec<SemanticDiagnostic>,
}