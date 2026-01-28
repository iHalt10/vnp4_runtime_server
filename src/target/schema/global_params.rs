use crate::target::schema::GlobalParamSchema;
use p4runtime::p4::config::v1::action::Param as P4RuntimeParam;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalParamsSchema(pub Vec<GlobalParamSchema>);

impl GlobalParamsSchema {
    pub fn get(&self, name: String) -> Option<GlobalParamSchema> {
        for param in self.0.iter() {
            if param.name == name {
                return Some(param.clone());
            }
        }
        return None;
    }

    pub fn as_p4info(&self) -> Vec<P4RuntimeParam> {
        return self.0.iter().map(|schema| schema.as_p4info()).collect();
    }
}

impl std::ops::Deref for GlobalParamsSchema {
    type Target = Vec<GlobalParamSchema>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for GlobalParamsSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
