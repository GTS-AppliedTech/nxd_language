#[derive(Debug)]
pub enum SemanticError {
    TypeMismatch {
        expected: String,
        actual: String,
    },

    UndefinedSymbol {
        name: String,
        line: u32,
        column: u32,
    },

    TraitNotImplemented {
        trait_name: String,
        ty: String,
    },

    CastFailure {
        from: String,
        to: String,
        reason: String,
    },
}

impl SemanticError {
    pub fn code(&self) -> &'static str {
        match self {
            SemanticError::UndefinedSymbol { .. } => {
                "NXD-S3001"
            }

            SemanticError::TypeMismatch { .. } => {
                "NXD-S3002"
            }

            SemanticError::TraitNotImplemented { .. } => {
                "NXD-S3003"
            }

            SemanticError::CastFailure { .. } => {
                "NXD-S3004"
            }
        }
    }

    pub fn line(&self) -> u32 {
        match self {
            SemanticError::UndefinedSymbol {
                line,
                ..
            } => *line,

            _ => 1,
        }
    }

    pub fn column(&self) -> u32 {
        match self {
            SemanticError::UndefinedSymbol {
                column,
                ..
            } => *column,

            _ => 1,
        }
    }

    pub fn message(&self) -> String {
        match self {
            SemanticError::UndefinedSymbol {
                name,
                ..
            } => {
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