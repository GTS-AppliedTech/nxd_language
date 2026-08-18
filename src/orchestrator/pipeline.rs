use crate::backend::nim::emitter;
use crate::ir::parse_ir::IRRoot;
use std::fs;

pub fn compile_from_ir_json(path: &str) -> Result<String, String> {
    let data = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;

    let ir_root: IRRoot = serde_json::from_str(&data)
        .map_err(|e| e.to_string())?;

    let nim_code = emitter::emit(&ir_root);

    Ok(nim_code)
}