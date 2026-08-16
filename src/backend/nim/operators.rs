use crate::ir::nodes::{IRBinaryOp, IRUnaryOp};

pub fn emit_binary_op(op: &IRBinaryOp) -> String {
    let left = &op.left;
    let right = &op.right;
    let op_str = map_binary_op(&op.kind);

    format!("{} {} {}", left, op_str, right)
}

pub fn emit_unary_op(op: &IRUnaryOp) -> String {
    let expr = &op.expr;
    let op_str = map_unary_op(&op.kind);

    format!("{} {}", op_str, expr)
}

fn map_binary_op(kind: &str) -> &str {
    match kind {
        "ADD" => "+",
        "SUB" => "-",
        "MUL" => "*",
        "DIV" => "/",
        "MOD" => "mod",
        "EQ"  => "==",
        "NEQ" => "!=",
        "GT"  => ">",
        "LT"  => "<",
        "GTE" => ">=",
        "LTE" => "<=",
        "AND" => "and",
        "OR"  => "or",
        _ => kind, // fallback
    }
}

fn map_unary_op(kind: &str) -> &str {
    match kind {
        "NOT" => "not",
        _ => kind,
    }
}

// Ownership / pipeline helpers

pub fn emit_move(expr: &str) -> String {
    // MOVE is a no-op in Nim for now
    expr.to_string()
}

pub fn emit_clone(expr: &str) -> String {
    format!("deepCopy({})", expr)
}

pub fn emit_borrow(expr: &str) -> String {
    // BORROW as reference semantics
    expr.to_string()
}

pub fn emit_pipeline(value: &str, func: &str) -> String {
    format!("{}({})", func.to_lowercase(), value)
}
