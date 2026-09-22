use crate::semantic::analyzer::Analyzer;
use crate::semantic::errors::SemanticError;
use crate::semantic::diagnostics::{
    SemanticDiagnostic,
    SemanticDiagnosticResponse,
};
use crate::semantic::traits::TraitRegistry;
use crate::backend::nim::emitter;
use crate::ir::parse_ir::IRRoot;
use std::fs;

fn load_ir_root(path: &str) -> Result<IRRoot, String>{
    let data = fs::read_to_string(path) .map_err(|e| e.to_string())?;

    serde_json::from_str(&data) .map_err(|e| e.to_string())
}
fn analyze_ir(ir_root: &IRRoot) -> Result<(), SemanticError> {
    let traits = TraitRegistry::new();
    let mut analyzer = Analyzer::new(traits);

    analyzer.analyze(ir_root)
}
pub fn compile_from_ir_json(
    path: &str,
) -> Result<String, String> {
    let ir_root = load_ir_root(path)?;

    let nim_code = emitter::emit(&ir_root);

    Ok(nim_code)
}
pub fn compile_from_ir_json_with_semantics(
    path: &str,
) -> Result<String, String> {
    let ir_root = load_ir_root(path)?;

    analyze_ir(&ir_root)
        .map_err(|error| {
            format!("Semantic error: {}", error.message())
        })?;

    let nim_code = emitter::emit(&ir_root);

    Ok(nim_code)
}
pub fn semantic_diagnostics_from_ir_json(
    path: &str,
) -> Result<SemanticDiagnosticResponse, String> {
    let ir_root = load_ir_root(path)?;

    match analyze_ir(&ir_root) {
        Ok(()) => Ok(
            SemanticDiagnosticResponse {
                ok: true,
                diagnostics: Vec::new(),
            }
        ),

        Err(error) => Ok(
            SemanticDiagnosticResponse {
                ok: false,
                diagnostics: vec![
                    SemanticDiagnostic {
                        code: error.code().to_string(),
                        message: error.message(),
                        line: 1,
                        column: 1,
                        severity: "error".to_string(),
                    }
                ],
            }
        ),
    }
}