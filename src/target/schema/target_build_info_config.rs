use crate::target::driver::XilVitisNetP4TargetBuildInfoConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetBuildInfoConfigSchema {
    #[serde(rename = "BaseAddr")]
    pub base_addr: usize,
}

impl TargetBuildInfoConfigSchema {
    pub fn from_driver_config(config: XilVitisNetP4TargetBuildInfoConfig) -> Self {
        Self { base_addr: config.BaseAddr }
    }

    pub fn to_driver_config(&self) -> XilVitisNetP4TargetBuildInfoConfig {
        XilVitisNetP4TargetBuildInfoConfig { BaseAddr: self.base_addr }
    }
}
