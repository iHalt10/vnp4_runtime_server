use crate::target::driver::XilVitisNetP4TargetTableConfig;
use crate::target::program::Program;
use crate::target::schema::MatchFieldsSchema;
use crate::target::schema::TableConfigSchema;
use crate::utils::serde::cstring as serde_cstring;
use serde::Deserialize;
use serde::Serialize;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use p4runtime::p4::config::v1::ActionRef as P4RuntimeActionRef;
use p4runtime::p4::config::v1::Preamble as P4RuntimePreamble;
use p4runtime::p4::config::v1::Table as P4RuntimeTable;
use p4runtime::p4::config::v1::TableActionCall as P4RuntimeTableActionCall;
use p4runtime::p4::config::v1::action_ref::Scope;
use p4runtime::p4::config::v1::table::IdleTimeoutBehavior;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetTableConfigSchema {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DefaultActionID")]
    pub default_action_id: u32,
    #[serde(rename = "MatchFieldList")]
    pub match_field_list: MatchFieldsSchema,
    #[serde(rename = "NameString")]
    #[serde(with = "serde_cstring")]
    pub name_string: CString,
    #[serde(rename = "Config")]
    pub config: TableConfigSchema,
}

impl TargetTableConfigSchema {
    pub fn from_driver_config(table: XilVitisNetP4TargetTableConfig, program: &Program) -> Self {
        let name = TargetTableConfigSchema::get_name(table);
        let pipeline_program = program.get_ingress_pipeline();
        let table_program = pipeline_program.get_table(name.to_string_lossy().to_string()).unwrap();
        Self {
            id: table_program.id,
            name: table_program.name,
            default_action_id: table_program.default_entry.action_id,
            match_field_list: table_program.match_fields.as_schema(program),
            name_string: name,
            config: TableConfigSchema::from_driver_config(table.Config, program.actions.as_schema()),
        }
    }

    pub fn get_name(table: XilVitisNetP4TargetTableConfig) -> CString {
        unsafe {
            let c_str = CStr::from_ptr(table.NameStringPtr);
            c_str.to_owned()
        }
    }

    pub fn to_driver_config(&mut self) -> XilVitisNetP4TargetTableConfig {
        let config = XilVitisNetP4TargetTableConfig {
            NameStringPtr: self.name_string.as_ptr() as *const c_char,
            Config: self.config.to_driver_config(),
        };
        return config;
    }

    pub fn as_p4info(&self) -> P4RuntimeTable {
        P4RuntimeTable {
            preamble: Some(P4RuntimePreamble {
                id: self.id,
                name: self.name.clone(),
                alias: "".to_string(),
                annotations: Vec::new(),
                annotation_locations: Vec::new(),
                doc: None,
                structured_annotations: Vec::new(),
            }),
            match_fields: self.match_field_list.as_p4info(),
            action_refs: self
                .config
                .action_list
                .keys()
                .map(|id| P4RuntimeActionRef {
                    id: *id,
                    scope: Scope::TableAndDefault.into(),
                    annotations: Vec::new(),
                    annotation_locations: Vec::new(),
                    structured_annotations: Vec::new(),
                })
                .collect(),
            const_default_action_id: 0,
            initial_default_action: Some(P4RuntimeTableActionCall {
                action_id: self.default_action_id,
                arguments: Vec::new(),
            }),
            implementation_id: 0,
            direct_resource_ids: Vec::new(),
            size: self.config.cam_config.num_entries as i64,
            idle_timeout_behavior: IdleTimeoutBehavior::NoTimeout.into(),
            is_const_table: false,
            has_initial_entries: false,
            other_properties: None,
        }
    }
}
