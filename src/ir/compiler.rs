use crate::diagnostics::warnings::{
    find_constant_conditions, find_redundant_match_arms, find_shadowed_bindings,
    find_unreachable_code, find_unused_functions, find_unused_parameters, find_unused_variables,
};
use crate::frontend::ast::nodes::AstNode;
use crate::frontend::parser::parser::parse;

use crate::ir::lowering::*;
use crate::ir::nodes::*;

pub fn compile_to_ir(src: &str) -> Result<IRRoot, Box<dyn std::error::Error>> {
    let (ast_module, ast_types, ast_functions) = parse(src)?;

    let mut warnings = find_unused_variables(&ast_module);
    warnings.extend(find_unreachable_code(&ast_module));
    warnings.extend(find_shadowed_bindings(&ast_module));
    warnings.extend(find_unused_parameters(&ast_module));
    warnings.extend(find_unused_functions(&ast_module));
    warnings.extend(find_constant_conditions(&ast_module));
    warnings.extend(find_redundant_match_arms(&ast_module));

    println!("\n=== AST ===");
    println!("{:#?}", ast_module);

    println!("\n=== WARNINGS ===");
    println!("{:#?}", warnings);

    let traits = ast_types
        .iter()
        .filter_map(|node| match node {
            AstNode::Trait(trait_node) => Some(lower_trait(trait_node)),
            _ => None,
        })
        .collect();

    let impls = ast_types
        .iter()
        .filter_map(|node| match node {
            AstNode::Impl(impl_node) => Some(lower_impl(impl_node)),
            _ => None,
        })
        .collect();

    let ir_root = IRRoot {
        module: lower_module(&ast_module),

        types: lower_types(&ast_types),

        traits,

        impls,

        functions: ast_functions.iter().map(lower_function).collect(),

        statements: Vec::new(),
    };

    Ok(ir_root)
}
