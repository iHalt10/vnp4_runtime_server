use crate::target::program::MatchField;
use crate::target::program::Program;
use crate::target::schema::MatchFieldSchema;
use crate::target::schema::MatchFieldsSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchFields(pub Vec<MatchField>);

impl MatchFields {
    pub fn as_schema(&self, program: &Program) -> MatchFieldsSchema {
        let mut match_field_list = MatchFieldsSchema(HashMap::new());

        for (index, match_field) in self.0.iter().enumerate() {
            let schema = MatchFieldSchema {
                id: index as u32,
                name: match_field.name.clone(),
                bitwidth: match_field.get_bitwidth(&program),
                match_type: match_field.match_type,
            };
            match_field_list.insert(index as u32, schema);
        }
        return match_field_list;
    }
}

impl std::ops::Deref for MatchFields {
    type Target = Vec<MatchField>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MatchFields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
