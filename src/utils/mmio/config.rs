use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MmioConfig {
    pub path: PathBuf,
    pub size: usize,
    pub offset: i64,
}
