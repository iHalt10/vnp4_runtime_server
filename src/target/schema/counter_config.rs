use serde::{Deserialize, Serialize};

use crate::target::driver::XilVitisNetP4CounterConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterConfigSchema {
    #[serde(rename = "BaseAddr")]
    pub base_addr: usize,
    #[serde(rename = "CounterType")]
    pub counter_type: u32,
    #[serde(rename = "NumCounters")]
    pub num_counters: u32,
    #[serde(rename = "Width")]
    pub width: u32,
}

impl CounterConfigSchema {
    pub fn from_driver_config(config: XilVitisNetP4CounterConfig) -> Self {
        Self {
            base_addr: config.BaseAddr,
            counter_type: config.CounterType,
            num_counters: config.NumCounters,
            width: config.Width,
        }
    }

    pub fn to_driver_config(&self) -> XilVitisNetP4CounterConfig {
        XilVitisNetP4CounterConfig {
            BaseAddr: self.base_addr,
            CounterType: self.counter_type,
            NumCounters: self.num_counters,
            Width: self.width,
        }
    }
}
