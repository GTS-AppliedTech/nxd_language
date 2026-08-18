use crate::ir::nodes::{IRFunction, IRStatement};

pub fn emit_functions(funcs: &Vec<IRFunction>) -> String {
    let mut out = String::new();

    for f in funcs {
        out.push_str(&emit_function(f));
        out.push('\n');
    }

    out
}

pub fn emit_function(f: &IRFunction) -> String {
    let mut out = String::new();

    // proc header
    out.push_str(&format!("proc {}(", f.name.to_lowercase()));

    // parameters
    let params: Vec<String> = f
        .params
        .iter()
        .map(|p| format!("{}: {}", p.name.to_lowercase(), p.ty))
        .collect();

    out.push_str(&params.join(", "));
    out.push_str(")");

    // return type
    if let Some(ret) = &f.return_type {
        out.push_str(&format!(": {}", ret));
    }

    out.push_str(" =\n");

    // body
    for stmt in &f.body {
        out.push_str(&emit_statement(stmt));
    }

    out
}

pub fn emit_statement(stmt: &IRStatement) -> String {
    match stmt {
        IRStatement::Let { name, value } => {
            format!("  var {} = {}\n", name.to_lowercase(), value)
        }

        IRStatement::Const { name, value } => {
            format!("  let {} = {}\n", name.to_lowercase(), value)
        }

        IRStatement::Return(expr) => {
            format!("  return {}\n", expr)
        }

        IRStatement::Loop(body) => {
            let mut out = String::new();
            out.push_str("  while true:\n");

            for stmt in body {
                out.push_str(&format!("    {}\n", emit_statement(stmt).trim()));
            }

            out
        }

        IRStatement::If(_) => {
            "  # TODO: emit if statement\n".to_string()
        }

        IRStatement::Match(_) => {
            "  # TODO: emit match statement\n".to_string()
        }

        IRStatement::Expr(expr) => {
            format!("  {}\n", expr)
        }
    }
}
