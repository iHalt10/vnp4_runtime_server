use crate::target::program::Field;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderType {
    pub id: u64,
    pub name: String,
    pub fields: Vec<Field>,
    pub is_struct: Option<bool>,
    pub length_exp: Option<String>,
    pub max_length: Option<u64>,
}

impl HeaderType {
    pub fn get_field(&self, name: String) -> Option<Field> {
        for field in self.fields.iter() {
            if field.name == name {
                return Some(field.clone());
            }
        }
        return None;
    }
}
