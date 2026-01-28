use crate::target::program::SourceInfo;
use crate::target::program::Table;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: u64,
    pub name: String,
    pub source_info: SourceInfo,
    pub init_table: String,
    pub tables: Vec<Table>,
    pub action_profiles: Value,
    pub conditionals: Value,
}

impl Pipeline {
    pub fn get_table(&self, name: String) -> Option<Table> {
        for table in self.tables.iter() {
            if table.name.ends_with(&name) {
                return Some(table.clone());
            }
        }
        return None;
    }
}
