use super::bridge::run_frontend;
use super::dispatcher::dispatch_backend;
use super::config::OrchestratorConfig;

pub fn run_pipeline(source_code: &str, config: OrchestratorConfig) -> String {
    // 1. Run frontend (Python)
    let frontend = run_frontend(source_code);

    // 2. IR → Backend
    let backend_output = dispatch_backend(&config, &frontend.ir_json);

    backend_output
}
