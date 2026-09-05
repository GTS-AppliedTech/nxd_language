use crate::ir::nodes::IRStatement;
use crate::backend::nim::operators::{emit_binary_op, emit_unary_op};
use crate::backend::nim::literals::emit_literal;
use crate::backend::nim::control_flow::{emit_if, emit_match};

pub fn emit_statements(stmts: &Vec<IRStatement>) -> String {
    let mut out = String::new();

    for stmt in stmts {
        out.push_str(&emit_statement(stmt));
    }

    out
}

pub fn emit_statement(stmt: &IRStatement) -> String {
    match stmt {
        IRStatement::Let { name, value } => {
            format!("  var {} = {}\n", name.to_lowercase(), emit_expr(value))
        }
        IRStatement::Const { name, value } => {
            format!("  let {} = {}\n", name.to_lowercase(), emit_expr(value))
        }
        IRStatement::Return(expr) => {
            format!("  return {}\n", emit_expr(expr))
        }
        IRStatement::Loop(body) => {
            let mut out = String::new();
            out.push_str("  while true:\n");
            for stmt in body {
                out.push_str(&indent(&emit_statement(stmt)));
            }
            out
        }
        IRStatement::If(if_node) => emit_if(if_node),
        IRStatement::Match(m) => emit_match(m),
        IRStatement::Try(t) => {
            let mut out = String::new();

            out.push_str("  # TRY\n");

            for stmt in &t.try_body {
                out.push_str(&indent(&emit_statement(stmt)));
            }

            out.push_str("  # CATCH\n");

            for stmt in &t.catch_body {
                out.push_str(&indent(&emit_statement(stmt)));
            }

            out.push_str("  # FINALLY\n");

            for stmt in &t.finally_body {
                out.push_str(&indent(&emit_statement(stmt)));
            }

            out
        }
        IRStatement::Move(movenode) => {
            format!(
                "  var {} = move({})\n",
                movenode.target.to_string().to_lowercase(),
                movenode.source.to_string().to_lowercase()
            )
        }
        IRStatement::Clone(clone_node) => {
            format!(
                "  var {} = deepCopy({})\n",
                clone_node.target.to_string().to_lowercase(),
                clone_node.source.to_string().to_lowercase()
            )
        }
        IRStatement::Expr(expr) => {
            format!("  {}\n", emit_expr(expr))
        }
    }
}


fn emit_expr(expr: &crate::ir::nodes::IRExpr) -> String {
    use crate::ir::nodes::IRExpr::*;

    match expr {
        Literal(l) => emit_literal(l),

        Binary(b) => emit_binary_op(b),

        Unary(u) => emit_unary_op(u),

        Call { func, args } => {
            let args_str: Vec<String> = args.iter().map(emit_expr).collect();

            let nim_func = match func.as_str() {
                "PRINTLN" => "echo".to_string(),
                _ => func.to_lowercase(),
            };

            format!("{}({})", nim_func, args_str.join(", "))
        }

        Var(name) => name.to_lowercase(),

        Pipeline { value, func } => {
            crate::backend::nim::operators::emit_pipeline(
                &emit_expr(value),
                func,
            )
        }

        ObjectInit(init) => {
            let mut fields = init.fields.iter().collect::<Vec<_>>();
            fields.sort_by(|a, b| a.0.cmp(b.0));

            let fields = fields
                .into_iter()
                .map(|(name, value)| {
                    format!("{}: {}", name, emit_expr(value))
                })
                .collect::<Vec<_>>()
                .join(", ");

            format!("{}({})", init.type_name, fields)
        }
    }
}

fn indent(s: &str) -> String {
    s.lines()
        .map(|line| format!("    {}\n", line.trim_end()))
        .collect()
}
