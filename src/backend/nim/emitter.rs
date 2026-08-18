use crate::ir::parse_ir::IRRoot;
use super::{module, types, traits, impls, functions};

pub fn emit(ir: &IRRoot) -> String {
    let mut out = String::new();

    out.push_str(&module::emit_module(&ir.module));
    out.push_str(&types::emit_types(&ir.types));
    out.push_str(&traits::emit_traits(&ir.traits));
    out.push_str(&impls::emit_impls(&ir.impls));
    out.push_str(&functions::emit_functions(&ir.functions));

    out
}