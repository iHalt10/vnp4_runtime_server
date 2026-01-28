use crate::target::program::MatchFieldTarget;
use crate::target::program::MatchType;
use crate::target::program::Program;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchField {
    pub match_type: MatchType,
    pub name: String,
    pub target: MatchFieldTarget,
    pub mask: Option<u64>,
}

impl MatchField {
    pub fn get_bitwidth(&self, program: &Program) -> i32 {
        let header = program.get_header(self.target.header_name.clone()).unwrap();
        let header_type = program.get_header_type(header.header_type).unwrap();
        let field = header_type.get_field(self.target.field_name.clone()).unwrap();
        return field.bitwidth;
    }
}
