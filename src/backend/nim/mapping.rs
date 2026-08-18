use crate::ir::nodes::*;
use crate::ir::parse_ir::IRRoot;
use crate::backend::nim::{
    module,
    types,
    traits,
    impls,
    functions,
    statements,
    control_flow,
    operators,
    literals,
};

pub fn emit_ir(ir: &IRRoot) -> String {
    let mut out = String::new();

    // MODULE
    out.push_str(&module::emit_module(&ir.module));

    // TYPES (STRUCT, ENUM, UNION, TRAIT)
    out.push_str(&types::emit_types(&ir.types));

    // TRAITS
    out.push_str(&traits::emit_traits(&ir.traits));

    // IMPLEMENTATIONS (IMPL blocks)
    out.push_str(&impls::emit_impls(&ir.impls));

    // FUNCTIONS
    out.push_str(&functions::emit_functions(&ir.functions));

    // TOP‑LEVEL STATEMENTS
    out.push_str(&statements::emit_statements(&ir.statements));

    out
}
