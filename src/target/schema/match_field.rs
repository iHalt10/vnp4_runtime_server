use crate::target::program::MatchType;
use serde::Deserialize;
use serde::Serialize;

use p4runtime::p4::config::v1::MatchField as P4RuntimeMatchField;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchFieldSchema {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "BitWidth")]
    pub bitwidth: i32,
    #[serde(rename = "MatchType")]
    pub match_type: MatchType,
}

impl MatchFieldSchema {
    pub fn as_p4info(&self) -> P4RuntimeMatchField {
        P4RuntimeMatchField {
            id: self.id,
            name: self.name.clone(),
            annotations: Vec::new(),
            annotation_locations: Vec::new(),
            bitwidth: self.bitwidth,
            doc: None,
            type_name: None,
            structured_annotations: Vec::new(),
            r#match: Some(self.match_type.as_match()),
        }
    }
}
