use crate::ir::nodes::{IRLiteral};

pub fn emit_literal(lit: &IRLiteral) -> String {
    match lit {
        IRLiteral::Int(v) => v.to_string(),
        IRLiteral::Float(v) => v.to_string(),
        IRLiteral::String(s) => format!("\"{}\"", s),
        IRLiteral::Bool(b) => {
            if *b { "true".to_string() } else { "false".to_string() }
        }
        IRLiteral::None => "nil".to_string(),
        IRLiteral::List(items) => emit_list(items),
    }
}

fn emit_list(items: &Vec<IRLiteral>) -> String {
    let inner: Vec<String> = items.iter().map(emit_literal).collect();
    format!("@[{}]", inner.join(", "))
}
