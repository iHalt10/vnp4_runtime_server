use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    pub filename: String,
    pub line: u64,
    pub column: u64,
    pub source_fragment: String,
}
