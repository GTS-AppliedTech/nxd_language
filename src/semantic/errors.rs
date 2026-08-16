#[derive(Debug)]
pub enum SemanticError {
    TypeMismatch { expected: String, actual: String },
    UndefinedSymbol { name: String },
    TraitNotImplemented { trait_name: String, ty: String },
    CastFailure { from: String, to: String, reason: String },
}
