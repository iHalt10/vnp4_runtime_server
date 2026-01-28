use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetInterruptConfigSchema {
    #[serde(rename = "BaseAddr")]
    pub base_addr: usize,
    #[serde(rename = "NumP4Elements")]
    pub num_p4_elements: u32,
    #[serde(rename = "NumComponents")]
    pub num_components: u32,
    #[serde(rename = "ComponentNameList")]
    pub component_name_list: Vec<String>,
}
