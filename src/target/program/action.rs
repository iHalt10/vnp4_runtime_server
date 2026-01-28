use crate::target::program::RuntimeData;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: u32,
    pub name: String,
    pub runtime_data: RuntimeData,
    pub primitives: Value,
}
