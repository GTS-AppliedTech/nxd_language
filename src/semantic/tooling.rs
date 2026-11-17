use std::collections::HashMap;

use crate::diagnostics::span::SourceSpan;

#[derive(Clone, Debug)]
pub struct ToolingSymbol {
    pub ty: String,
    pub span: SourceSpan,
}

#[derive(Clone, Debug)]
pub struct ToolingMember {
    pub object_type: String,
    pub member: String,
    pub ty: String,
    pub span: SourceSpan,
}

#[derive(Clone, Debug)]
pub struct FunctionToolingInfo {
    pub span: SourceSpan,
    pub variables: HashMap<String, ToolingSymbol>,
    pub constants: HashMap<String, ToolingSymbol>,
    pub members: Vec<ToolingMember>,
}

#[derive(Clone, Debug, Default)]
pub struct ToolingAnalysis {
    pub functions: HashMap<String, FunctionToolingInfo>,
    pub types: HashMap<String, HashMap<String, String>>,
}
