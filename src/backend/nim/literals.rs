use crate::backend::nim::statements::emit_expr;
use crate::ir::nodes::*;

pub fn emit_literal(lit: &IRLiteral) -> String {
    match lit {
        IRLiteral::Int(v) => v.to_string(),
        IRLiteral::Float(v) => v.to_string(),
        IRLiteral::String(s) => format!("\"{}\"", s),
        IRLiteral::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        IRLiteral::None => "nil".to_string(),
        IRLiteral::List(items) => emit_list(items),
        IRLiteral::Map(_) => {
            unimplemented!("Nim emission for map literals is not implemented")
        }

        IRLiteral::Lambda { .. } => {
            unimplemented!("Nim emission for lambda literals is not implemented")
        }
    }
}

fn emit_list(items: &Vec<IRExpr>) -> String {
    let inner: Vec<String> = items.iter().map(emit_expr).collect();
    format!("@[{}]", inner.join(", "))
}
