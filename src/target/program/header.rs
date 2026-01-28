use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub id: u64,
    pub name: String,
    pub header_type: String,
    pub metadata: bool,
    pub pi_omit: Option<bool>,
}
