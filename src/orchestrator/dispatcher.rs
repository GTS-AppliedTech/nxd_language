use crate::backend::nim::emitter as nim_backend;
use crate::backend::d::emitter as d_backend;
use crate::backend::elixir::emitter as elixir_backend;

use super::config::{OrchestratorConfig, TargetBackend};

pub fn dispatch_backend(config: &OrchestratorConfig, ir_json: &str) -> String {
    match config.backend {
        TargetBackend::Nim => nim_backend::emit(ir_json),
        TargetBackend::D => d_backend::emit(ir_json),
        TargetBackend::Elixir => elixir_backend::emit(ir_json),
    }
}
