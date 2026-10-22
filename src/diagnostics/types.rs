use crate::diagnostics::severity::DiagnosticSeverity;
use crate::diagnostics::span::SourceSpan;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub code: String,
    pub message: String,
    pub span: SourceSpan,
    pub severity: DiagnosticSeverity,
}

#[derive(Debug, Clone)]
pub struct WarningDiagnostic {
    pub code: String,
    pub message: String,
    pub span: SourceSpan,
    pub severity: DiagnosticSeverity,
}
