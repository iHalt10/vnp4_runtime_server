use p4runtime::p4::config::v1::action::Param as P4RuntimeParam;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalParamSchema {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "BitWidth")]
    pub bitwidth: i32,
}

impl GlobalParamSchema {
    pub fn as_p4info(&self) -> P4RuntimeParam {
        P4RuntimeParam {
            id: self.id,
            name: self.name.clone(),
            annotations: Vec::new(),
            annotation_locations: Vec::new(),
            bitwidth: self.bitwidth,
            doc: None,
            type_name: None,
            structured_annotations: Vec::new(),
        }
    }
}
