pub struct FrontendOutput {
    pub ast_json: String,
    pub ir_json: String,
}

pub fn run_frontend(source: &str) -> FrontendOutput {
    // placeholder for Python call
    FrontendOutput {
        ast_json: "{}".to_string(),
        ir_json: "{}".to_string(),
    }
}
