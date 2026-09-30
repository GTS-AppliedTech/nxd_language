

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

pub fn task_type(inner: &str) -> String {
    format!("Task[{}]", inner)
}

pub fn unwrap_task_type(ty: &str) -> Option<String> {
    ty.strip_prefix("Task[")
        .and_then(|inner| inner.strip_suffix(']'))
        .map(|inner| inner.to_string())
}

pub fn unwrap_awaitable_type(ty: &str) -> Option<String> {
    unwrap_task_type(ty)
        .or_else(|| unwrap_process_handle_type(ty))
        .or_else(|| unwrap_receive_operation_type(ty))
}

pub fn receive_operation_type(inner: &str) -> String {
    format!("ReceiveOperation[{}]", inner)
}

pub fn unwrap_receive_operation_type(
    ty: &str,
) -> Option<String> {
    ty.strip_prefix("ReceiveOperation[")
        .and_then(|inner| inner.strip_suffix(']'))
        .map(|inner| inner.to_string())
}

pub fn process_handle_type(inner: &str) -> String {
    format!("ProcessHandle[{}]", inner)
}

pub fn unwrap_process_handle_type(ty: &str) -> Option<String> {
    ty.strip_prefix("ProcessHandle[")
        .and_then(|inner| inner.strip_suffix(']'))
        .map(|inner| inner.to_string())
}

fn is_primitive(t: &str) -> bool {
    matches!(t, "int" | "float" | "string" | "bool" | "none")
}
