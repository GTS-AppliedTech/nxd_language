use crate::ir::nodes::IRTrait;

pub fn emit_traits(traits: &Vec<IRTrait>) -> String {
    let mut out = String::new();

    if traits.is_empty() {
        return out;
    }

    out.push_str("type\n");

    for tr in traits {
        out.push_str(&emit_trait(tr));
    }

    out.push('\n');
    out
}

fn emit_trait(tr: &IRTrait) -> String {
    let mut out = String::new();

    out.push_str(&format!("  {} = concept x\n", tr.name));

    for func in &tr.methods {
        out.push_str(&format!(
            "    {}(x) is {}\n",
            func.name.to_lowercase(),
            func.return_type
        ));
    }

    out
}
