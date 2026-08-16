use crate::ir::nodes::{IRIf, IRMatch, IRStatement};

pub fn emit_if(if_node: &IRIf) -> String {
    let mut out = String::new();

    out.push_str(&format!("if {}:\n", if_node.condition));
    for stmt in &if_node.then_branch {
        out.push_str(&indent(&emit_statement(stmt)));
    }

    if !if_node.else_branch.is_empty() {
        out.push_str("else:\n");
        for stmt in &if_node.else_branch {
            out.push_str(&indent(&emit_statement(stmt)));
        }
    }

    out
}

pub fn emit_match(m: &IRMatch) -> String {
    let mut out = String::new();

    out.push_str(&format!("case {}:\n", m.scrutinee));

    for arm in &m.arms {
        out.push_str(&format!("  of {}:\n", arm.pattern));
        for stmt in &arm.body {
            out.push_str(&indent(&emit_statement(stmt)));
        }
    }

    if let Some(otherwise) = &m.otherwise {
        out.push_str("  else:\n");
        for stmt in otherwise {
            out.push_str(&indent(&emit_statement(stmt)));
        }
    }

    out
}

// Helper: reuse statement emitter from functions.rs
fn emit_statement(stmt: &IRStatement) -> String {
    // You can re-export or call the existing statement emitter here.
    // For now, assume a shared function:
    crate::backend::nim::functions::emit_statement(stmt)
}

fn indent(s: &str) -> String {
    s.lines()
        .map(|line| format!("  {}\n", line.trim_end()))
        .collect()
}
