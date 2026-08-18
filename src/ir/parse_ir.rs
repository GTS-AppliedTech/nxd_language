use serde::Deserialize;
use crate::ir::nodes::*;

#[derive(Deserialize)]
pub struct IRRoot {
    pub module: IRModule,

    #[serde(default)]
    pub types: Vec<IRTypeDecl>,

    #[serde(default)]
    pub traits: Vec<IRTrait>,

    #[serde(default)]
    pub impls: Vec<IRImpl>,

    #[serde(default)]
    pub functions: Vec<IRFunction>,

    #[serde(default)]
    pub statements: Vec<IRStatement>,
}

pub fn parse_ir(json: &str) -> IRRoot {
    serde_json::from_str(json)
        .expect("Failed to parse IR JSON")
}
