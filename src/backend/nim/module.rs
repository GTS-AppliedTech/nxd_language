use crate::ir::nodes::{IRModule, IRImport};

pub fn emit_module(module: &IRModule) -> String {
    let mut out = String::new();

    // 1. Module header (Nim uses file-level modules)
    out.push_str(&format!("# {}\n\n", module.name.to_lowercase()));

    // 2. Imports
    for imp in &module.imports {
        out.push_str(&emit_import(imp));
    }

    out.push('\n');
    out
}

fn emit_import(imp: &IRImport) -> String {
    match &imp.alias {
        Some(alias) => format!("import {} as {}\n", imp.path.to_lowercase(), alias.to_lowercase()),
        None => format!("import {}\n", imp.path.to_lowercase()),
    }
}
