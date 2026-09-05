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

        IRStatement::Move(movenode) => {
            format!(
                "  var {} = move({})\n",
                movenode.target,
                movenode.source
            )
        }

        IRStatement::Clone(clone_node) => {
            format!(
                "  var {} = deepCopy({})\n",
                clone_node.target.to_string().to_lowercase(),
                clone_node.source.to_string().to_lowercase()
            )
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
        // Keep your remaining existing arms here
        IRStatement::If(if_node) => {
    crate::backend::nim::control_flow::emit_if(if_node)
}

        IRStatement::Match(m) => {
    crate::backend::nim::control_flow::emit_match(m)
}
        IRStatement::Try(t) => {
            let mut out = String::new();
            out.push_str("  # TRY\n");
            for stmt in &t.try_body {
                out.push_str(&emit_statement(stmt));
            }
            out.push_str("  # CATCH\n");
            for stmt in &t.catch_body {
                out.push_str(&emit_statement(stmt));
            }
            out.push_str("  # FINALLY\n");
            for stmt in &t.finally_body {
                out.push_str(&emit_statement(stmt));
            }

            out
        }
        IRStatement::Expr(expr) => {
            format!("  {}\n", expr)
        }
    }
}
