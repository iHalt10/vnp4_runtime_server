use crate::target::program::Actions;
use crate::target::program::Header;
use crate::target::program::HeaderType;
use crate::target::program::Pipeline;
use crate::utils::serde::JsonError;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub program: String,
    pub __meta__: Value,
    pub header_types: Vec<HeaderType>,
    pub headers: Vec<Header>,
    pub header_stacks: Value,
    pub header_union_types: Value,
    pub header_unions: Value,
    pub header_union_stacks: Value,
    pub field_lists: Value,
    pub errors: Value,
    pub enums: Value,
    pub parsers: Value,
    pub parse_vsets: Value,
    pub deparsers: Value,
    pub meter_arrays: Value,
    pub counter_arrays: Value,
    pub register_arrays: Value,
    pub calculations: Value,
    pub learn_lists: Value,
    pub actions: Actions,
    pub pipelines: Vec<Pipeline>,
    pub checksums: Value,
    pub internet_checksums: Value,
    pub force_arith: Value,
    pub extern_instances: Value,
    pub field_aliases: Value,
    pub p4_design_id: Value,
}

impl Program {
    pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Self, JsonError> {
        let path = path.as_ref();
        let file = File::open(&path).map_err(|e| JsonError::FileRead { path: path.to_path_buf(), source: e })?;
        let reader = BufReader::new(file);
        let program: Self = serde_json::from_reader(reader)?;
        Ok(program)
    }

    pub fn get_pipeline(&self, name: String) -> Option<Pipeline> {
        for pipeline in self.pipelines.iter() {
            if pipeline.name == name {
                return Some(pipeline.clone());
            }
        }
        return None;
    }

    pub fn get_ingress_pipeline(&self) -> Pipeline {
        let pipeline = self.get_pipeline("ingress".to_string());
        return pipeline.unwrap();
    }

    pub fn get_header(&self, name: String) -> Option<Header> {
        for header in self.headers.iter() {
            if header.name == name {
                return Some(header.clone());
            }
        }
        return None;
    }

    pub fn get_header_type(&self, name: String) -> Option<HeaderType> {
        for header_type in self.header_types.iter() {
            if header_type.name == name {
                return Some(header_type.clone());
            }
        }
        return None;
    }
}
