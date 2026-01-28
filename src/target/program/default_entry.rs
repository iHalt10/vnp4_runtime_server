use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultEntry {
    pub action_id: u32,
    pub action_const: Value,
    pub action_data: Value,
    pub action_entry_const: Value,
}
