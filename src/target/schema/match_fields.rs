use crate::target::schema::MatchFieldSchema;
use p4runtime::p4::config::v1::MatchField as P4RuntimeMatchField;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchFieldsSchema(pub HashMap<u32, MatchFieldSchema>);

impl MatchFieldsSchema {
    pub fn as_p4info(&self) -> Vec<P4RuntimeMatchField> {
        return self.0.values().map(|schema| schema.as_p4info()).collect();
    }
}

impl std::ops::Deref for MatchFieldsSchema {
    type Target = HashMap<u32, MatchFieldSchema>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MatchFieldsSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
