use crate::ir::nodes::{IRTypeDecl, IRStruct, IREnum, IRUnion, IRTrait};

pub fn emit_types(types: &Vec<IRTypeDecl>) -> String {
    let mut out = String::new();

    if types.is_empty() {
        return out;
    }

    out.push_str("type\n");

    for t in types {
        match t {
            IRTypeDecl::Struct(s) => out.push_str(&emit_struct(s)),
            IRTypeDecl::Enum(e) => out.push_str(&emit_enum(e)),
            IRTypeDecl::Union(u) => out.push_str(&emit_union(u)),
            IRTypeDecl::Trait(tr) => out.push_str(&emit_trait(tr)),
        }
    }

    out.push('\n');
    out
}

fn emit_struct(s: &IRStruct) -> String {
    let mut out = String::new();

    out.push_str(&format!("  {} = object\n", s.name));
    for field in &s.fields {
        out.push_str(&format!("    {}: {}\n", field.name.to_lowercase(), field.ty));
    }

    out
}

fn emit_enum(e: &IREnum) -> String {
    let mut out = String::new();

    out.push_str(&format!("  {} = enum\n", e.name));
    for variant in &e.variants {
        out.push_str(&format!("    {},\n", variant));
    }

    out
}

fn emit_union(u: &IRUnion) -> String {
    let mut out = String::new();

    out.push_str(&format!("  {} = object\n", u.name));
    out.push_str("    case kind: ResultKind\n");

    for variant in &u.variants {
        out.push_str(&format!("    of {}:\n", variant.kind));
        for field in &variant.fields {
            out.push_str(&format!("      {}: {}\n", field.name.to_lowercase(), field.ty));
        }
    }

    out
}

fn emit_trait(tr: &IRTrait) -> String {
    let mut out = String::new();

    out.push_str(&format!("  {} = concept x\n", tr.name));

    for func in &tr.methods {
        out.push_str(&format!("    {}(x) is {}\n", func.name.to_lowercase(), func.return_type));
    }

    out
}
