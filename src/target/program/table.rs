use crate::target::program::DefaultEntry;
use crate::target::program::MatchFields;
use crate::target::program::MatchType;
use crate::target::program::SourceInfo;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: u32,
    pub name: String,
    pub source_info: Option<SourceInfo>,
    pub sequence_point: Option<bool>,
    pub match_type: MatchType,
    pub max_size: i64,
    pub with_counters: bool,
    pub support_timeout: bool,
    pub direct_meters: Value,
    pub action_ids: Value,
    pub actions: Value,
    pub base_default_next: Value,
    pub next_tables: Value,
    pub default_entry: DefaultEntry,

    #[serde(rename = "key")]
    pub match_fields: MatchFields,
    #[serde(rename = "type")]
    pub implementation_type: String,
}
