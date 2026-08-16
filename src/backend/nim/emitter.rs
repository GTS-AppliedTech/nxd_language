pub fn emit(ir_json: &str) -> String {
    // TODO: parse IR JSON and generate Nim source
    "// Nim backend not implemented yet\n".to_string()
}
pub fn emit(ir_json: &str) -> String {
    let ir = parse_ir(ir_json);

    let mut out = String::new();

    out.push_str(&module::emit_module(&ir.module));
    out.push_str(&types::emit_types(&ir.types));
    out.push_str(&traits::emit_traits(&ir.traits));
    out.push_str(&impls::emit_impls(&ir.impls));
    out.push_str(&functions::emit_functions(&ir.functions));

    out
}
