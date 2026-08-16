use serde::Deserialize;
use crate::ir::nodes::*;

#[derive(Deserialize)]
pub struct IRRoot {
    pub module: IRModule,
    pub types: Vec<IRTypeDecl>,
    pub traits: Vec<IRTrait>,
    pub impls: Vec<IRImpl>,
    pub functions: Vec<IRFunction>,
    pub statements: Vec<IRStatement>,
}

pub fn parse_ir(json: &str) -> IRRoot {
    serde_json::from_str(json)
        .expect("Failed to parse IR JSON")
}
