use super::error::ConfigError;
use crate::utils::mmio::MmioConfig;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub devices: Vec<DeviceConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub id: u64,
    pub mmio: MmioConfig,

    #[serde(rename = "cpuPort")]
    pub cpu_port: String,

    #[serde(rename = "targetConfig")]
    pub target_config: PathBuf,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let file = File::open(&path).map_err(|e| ConfigError::FileRead { path: path.to_path_buf(), source: e })?;
        let reader = BufReader::new(file);
        let config: Self = serde_yaml::from_reader(reader)?;
        Ok(config)
    }
}
