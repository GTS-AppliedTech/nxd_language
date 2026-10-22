pub mod pipeline;

pub use pipeline::compile_from_ir;
pub use pipeline::compile_from_ir_with_semantics;
pub use pipeline::definition_from_ir;
pub use pipeline::semantic_diagnostics_from_ir;
