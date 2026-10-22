use crate::backend::nim::emitter as nim_backend;
use crate::backend::d::emitter as d_backend;
use crate::backend::elixir::emitter as elixir_backend;

use super::config::{OrchestratorConfig, TargetBackend};
use crate::ir::nodes::IRRoot;

pub fn dispatch_backend(config: &OrchestratorConfig, ir: &IRRoot) -> Result<String, String> {
    match config.backend {
        TargetBackend::Nim => Ok(nim_backend::emit(ir)),
        TargetBackend::D => Ok(d_backend::emit(ir)),
        TargetBackend::Elixir => Ok(elixir_backend::emit(ir)),
    }
}
