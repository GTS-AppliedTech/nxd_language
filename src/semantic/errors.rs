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
        end_line: u32,
        end_column: u32,
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
    InvalidAsyncUsage {
        violation: AsyncViolation,
    }
}

#[derive(Debug)]
pub enum AsyncViolation {
    AwaitOutsideAsync,      // S3005-01
    AwaitNonAwaitable,      // S3005-02
    InvalidSpawnTarget,     // S3005-03
    InvalidSendTarget,      // S3005-04
    InvalidRecvSource,      // S3005-05
    AsyncContextViolation,  // S3005-06
    InvalidPingTarget,      // S3005-07
    InvalidEscTarget,       // S3005-08
    InvalidTimeoutTarget,   // S3005-09
}

impl AsyncViolation {

    pub fn code(&self) -> &'static str {
        match self {
            AsyncViolation::AwaitOutsideAsync =>
                "NXD-S3005-01",

            AsyncViolation::AwaitNonAwaitable =>
                "NXD-S3005-02",

            AsyncViolation::InvalidSpawnTarget =>
                "NXD-S3005-03",

            AsyncViolation::InvalidSendTarget =>
                "NXD-S3005-04",

            AsyncViolation::InvalidRecvSource =>
                "NXD-S3005-05",

            AsyncViolation::AsyncContextViolation =>
                "NXD-S3005-06",

            AsyncViolation::InvalidPingTarget =>
                "NXD-S3005-07",

            AsyncViolation::InvalidEscTarget =>
                "NXD-S3005-08",

            AsyncViolation::InvalidTimeoutTarget =>
                "NXD-S3005-09",
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            AsyncViolation::AwaitOutsideAsync =>
                "AWAIT may only be used inside an ASYNC FUNC",

            AsyncViolation::AwaitNonAwaitable =>
                "Cannot AWAIT non-awaitable expression",

            AsyncViolation::InvalidSpawnTarget =>
                "SPAWN requires callable async target",

            AsyncViolation::InvalidSendTarget =>
                "SEND target is not a channel",

            AsyncViolation::InvalidRecvSource =>
                "RECV source is not a channel",

            AsyncViolation::AsyncContextViolation =>
                "Async operation is not permitted in the current execution context",

            AsyncViolation::InvalidPingTarget =>
                "PING target is not a ProcessHandle",

            AsyncViolation::InvalidEscTarget =>
                "Invalid ESC target",

            AsyncViolation::InvalidTimeoutTarget =>
                "Invalid TIMEOUT target",
        }
    }
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

        // ==================================================
        // ========== S3003 exists but generic constraints ==
        // ========== are not implemented yet. ==============
        // ==================================================

            SemanticError::TraitNotImplemented { .. } => {
                "NXD-S3003"
            }

            SemanticError::CastFailure { .. } => {
                "NXD-S3004"
            }

            SemanticError::InvalidAsyncUsage { .. } => {
                "NXD-S3005"
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

    pub fn end_line(&self) -> u32 {
        match self {
            SemanticError::UndefinedSymbol {
                end_line,
                ..
            } => *end_line,

            _ => 1,
        }
    }

    pub fn end_column(&self) -> u32 {
        match self {
            SemanticError::UndefinedSymbol {
                end_column,
                ..
            } => *end_column,

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
            SemanticError::InvalidAsyncUsage {
                violation,
            } => {
                format!(
                    "{}: {}",
                    violation.code(),
                    violation.message(),
                )
            }
        }
    }
}