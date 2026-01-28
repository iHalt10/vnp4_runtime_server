use crate::target::program::Param;
use crate::target::schema::GlobalParamSchema;
use crate::target::schema::GlobalParamsSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeData(pub Vec<Param>);

impl RuntimeData {
    pub fn as_schema(&self) -> GlobalParamsSchema {
        let mut params = GlobalParamsSchema(Vec::new());
        for (index, param) in self.0.iter().enumerate() {
            let schema = GlobalParamSchema {
                id: index as u32,
                name: param.name.clone(),
                bitwidth: param.bitwidth,
            };
            params.push(schema);
        }
        return params;
    }
}

impl std::ops::Deref for RuntimeData {
    type Target = Vec<Param>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for RuntimeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
