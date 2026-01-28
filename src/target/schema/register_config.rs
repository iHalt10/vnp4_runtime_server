use serde::{Deserialize, Serialize};

use crate::target::driver::XilVitisNetP4RegisterTopConfig;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterConfigSchema {
    #[serde(rename = "BaseAddr")]
    pub base_addr: usize,
    #[serde(rename = "version")]
    pub version: u16,
    #[serde(rename = "table_id")]
    pub table_id: u16,
    #[serde(rename = "largest_index")]
    pub largest_index: u32,
    #[serde(rename = "data_size")]
    pub data_size: u16,
    #[serde(rename = "InitialData")]
    pub initial_data: Vec<u32>,
    #[serde(rename = "dram")]
    pub dram: bool,
}

impl RegisterConfigSchema {
    pub fn from_driver_config(config: XilVitisNetP4RegisterTopConfig) -> Self {
        Self {
            base_addr: config.BaseAddr,
            version: config.version,
            table_id: config.table_id,
            largest_index: config.largest_index,
            data_size: config.data_size,
            initial_data: config.InitialData.to_vec(),
            dram: config.dram,
        }
    }

    pub fn to_driver_config(&self) -> XilVitisNetP4RegisterTopConfig {
        let mut initial_data = [0u32; 128];
        let len = self.initial_data.len().min(128);
        initial_data[..len].copy_from_slice(&self.initial_data[..len]);
        XilVitisNetP4RegisterTopConfig {
            BaseAddr: self.base_addr,
            version: self.version,
            table_id: self.table_id,
            largest_index: self.largest_index,
            data_size: self.data_size,
            InitialData: initial_data,
            dram: self.dram,
        }
    }
}
