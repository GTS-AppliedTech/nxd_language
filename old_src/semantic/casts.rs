use crate::semantic::errors::SemanticError;

pub fn check_as_cast(from: &str, to: &str) -> Result<(), SemanticError> {
    match (from, to) {
        ("int", "float") => Ok(()),
        ("float", "int") => Err(SemanticError::CastFailure {
            from: from.to_string(),
            to: to.to_string(),
            reason: "float → int requires no fractional part".to_string(),
        }),
        ("string", "int") => Err(SemanticError::CastFailure {
            from: from.to_string(),
            to: to.to_string(),
            reason: "string must be numeric".to_string(),
        }),
        ("int", "string") => Ok(()),
        ("enum", "string") => Ok(()),
        ("none", _) if to.starts_with("option") => Ok(()),
        _ => Err(SemanticError::CastFailure {
            from: from.to_string(),
            to: to.to_string(),
            reason: "illegal conversion".to_string(),
        }),
    }
}

pub fn check_is(from: &str, to: &str) -> bool {
    from == to
}
