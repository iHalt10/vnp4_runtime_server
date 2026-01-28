use crate::target::schema::TargetTableConfigSchema;
use p4runtime::p4::config::v1::Table as P4RuntimeTable;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetTableConfigsSchema(pub HashMap<u32, TargetTableConfigSchema>);

impl TargetTableConfigsSchema {
    pub fn as_p4info(&self) -> Vec<P4RuntimeTable> {
        return self.0.values().map(|schema| schema.as_p4info()).collect();
    }
}

impl std::ops::Deref for TargetTableConfigsSchema {
    type Target = HashMap<u32, TargetTableConfigSchema>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TargetTableConfigsSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
