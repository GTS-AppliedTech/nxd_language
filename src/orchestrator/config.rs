#[derive(Debug, Clone)]
pub enum TargetBackend {
    Nim,
    D,
    Elixir,
}

#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    pub backend: TargetBackend,
    pub optimize: bool,
    pub debug: bool,
}

impl OrchestratorConfig {
    pub fn default() -> Self {
        Self {
            backend: TargetBackend::Nim,
            optimize: false,
            debug: false,
        }
    }
}
