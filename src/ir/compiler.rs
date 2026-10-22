use crate::diagnostics::warnings::find_shadowed_bindings;
use crate::diagnostics::warnings::find_unreachable_code;
use crate::diagnostics::warnings::find_unused_variables;
use crate::frontend::parser::parser::parse;

use crate::ir::lowering::*;
use crate::ir::nodes::*;

use serde_json::to_string_pretty;

pub fn compile_to_ir(src: &str) -> Result<IRRoot, Box<dyn std::error::Error>> {
    let (ast_module, ast_types, ast_functions) = parse(src)?;

    let mut warnings = find_unused_variables(&ast_module);
    warnings.extend(find_unreachable_code(&ast_module));
    warnings.extend(find_shadowed_bindings(&ast_module));

    println!("\n=== AST ===");
    println!("{:#?}", ast_module);

    println!("\n=== WARNINGS ===");
    println!("{:#?}", warnings);

    let ir_root = IRRoot {
        module: lower_module(&ast_module),

        types: lower_types(&ast_types),

        traits: Vec::new(),

        impls: Vec::new(),

        functions: ast_functions.iter().map(lower_function).collect(),

        statements: Vec::new(),
    };

    Ok(ir_root)
}

pub fn compile_to_ir_json(src: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ir_root = compile_to_ir(src)?;

    let json = to_string_pretty(&ir_root)?;

    Ok(json)
}
