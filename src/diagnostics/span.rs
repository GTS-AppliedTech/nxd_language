use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceSpan {
    pub line: u32,
    pub column: u32,
    pub end_line: u32,
    pub end_column: u32,
}
