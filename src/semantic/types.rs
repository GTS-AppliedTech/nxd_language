use std::any;

use crate::semantic::errors::SemanticError;

pub fn check_type(expected: &str, actual: &str) -> Result<(), SemanticError> {
    if expected == "any" || actual == "any" {
        return Ok(());
    }
    if expected == actual {
        return Ok(());
    }
    // primitive mismatch
    if is_primitive(expected) && is_primitive(actual) {
        return Err(SemanticError::TypeMismatch {
            expected: expected.to_string(),
            actual: actual.to_string(),
        });
    }

    // TODO: generic resolution
    // TODO: trait-based type compatibility

    Err(SemanticError::TypeMismatch {
        expected: expected.to_string(),
        actual: actual.to_string(),
    })
}

fn is_primitive(t: &str) -> bool {
    matches!(t, "int" | "float" | "string" | "bool" | "none")
}
