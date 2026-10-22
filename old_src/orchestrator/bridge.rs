use crate::ir::parse_ir::load_ir_from_json;
//use crate::semantic::analyzer::SemanticAnalyzer;
use crate::orchestrator::dispatcher::dispatch_backend;
use crate::orchestrator::config::OrchestratorConfig;

pub fn run_frontend(ir_json_path: &str, config: &OrchestratorConfig) -> Result<String, String> {
    // 1. Load IR JSON → Rust IR structs
    let ir_root = load_ir_from_json(ir_json_path)
        .map_err(|e| format!("Failed to load IR: {e}"))?;

    // 2. Run semantic analysis
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ir_root.module, &ir_root.types, &ir_root.functions)
        .map_err(|e| format!("Semantic error: {e}"))?;

    // 3. Dispatch backend
    let output = dispatch_backend(config, &ir_root)
        .map_err(|e| format!("Backend error: {e}"))?;

    Ok(output)
}
