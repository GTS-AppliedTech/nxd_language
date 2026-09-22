#[derive(Debug)]
pub enum SemanticError {
    TypeMismatch { expected: String, actual: String },
    UndefinedSymbol { name: String },
    TraitNotImplemented { trait_name: String, ty: String },
    CastFailure { from: String, to: String, reason: String },
}

impl SemanticError {
    pub fn message(&self) -> String {
        match self {
            SemanticError::UndefinedSymbol { name } => {
                format!(
                    "NXD-S3001: Undefined symbol '{}'",
                    name
                )
            }

            SemanticError::TypeMismatch {
                expected,
                actual,
            } => {
                format!(
                    "NXD-S3002: Type mismatch (expected {}, got {})",
                    expected,
                    actual
                )
            }

            SemanticError::TraitNotImplemented {
                trait_name,
                ty,
            } => {
                format!(
                    "NXD-S3003: Trait '{}' not implemented for '{}'",
                    trait_name,
                    ty
                )
            }

            SemanticError::CastFailure {
                from,
                to,
                reason,
            } => {
                format!(
                    "NXD-S3004: Invalid cast from '{}' to '{}': {}",
                    from,
                    to,
                    reason
                )
            }
        }
    }
}
