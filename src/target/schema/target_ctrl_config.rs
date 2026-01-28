use crate::target::driver::XilVitisNetP4TargetCtrlConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetCtrlConfigSchema {
    #[serde(rename = "BaseAddr")]
    pub base_addr: usize,
    #[serde(rename = "NumP4Elements")]
    pub num_p4_elements: u32,
    #[serde(rename = "NumComponents")]
    pub num_components: u32,
    #[serde(rename = "ClkInHz")]
    pub clk_in_hz: u32,
    #[serde(rename = "PktRatePerSec")]
    pub pkt_rate_per_sec: u32,
}

impl TargetCtrlConfigSchema {
    pub fn from_driver_config(config: XilVitisNetP4TargetCtrlConfig) -> Self {
        Self {
            base_addr: config.BaseAddr,
            num_p4_elements: config.NumP4Elements,
            num_components: config.NumComponents,
            clk_in_hz: config.ClkInHz,
            pkt_rate_per_sec: config.PktRatePerSec,
        }
    }

    pub fn to_driver_config(&self) -> XilVitisNetP4TargetCtrlConfig {
        XilVitisNetP4TargetCtrlConfig {
            BaseAddr: self.base_addr,
            NumP4Elements: self.num_p4_elements,
            NumComponents: self.num_components,
            ClkInHz: self.clk_in_hz,
            PktRatePerSec: self.pkt_rate_per_sec,
        }
    }
}
