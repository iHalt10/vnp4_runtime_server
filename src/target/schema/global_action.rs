use crate::target::schema::GlobalParamsSchema;
use serde::Deserialize;
use serde::Serialize;

use p4runtime::p4::config::v1::Action as P4RuntimeAction;
use p4runtime::p4::config::v1::Preamble as P4RuntimePreamble;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalActionSchema {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "GlobalParams")]
    pub params: GlobalParamsSchema,
}

impl GlobalActionSchema {
    pub fn as_p4info(&self) -> P4RuntimeAction {
        P4RuntimeAction {
            preamble: Some(P4RuntimePreamble {
                id: self.id,
                name: self.name.clone(),
                alias: "".to_string(),
                annotations: Vec::new(),
                annotation_locations: Vec::new(),
                doc: None,
                structured_annotations: Vec::new(),
            }),
            params: self.params.as_p4info(),
        }
    }
}
