use super::error::RunServerProcessError;
use crate::server::config::Config;
use crate::server::service::P4RuntimeService;
use crate::target::models::Device;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::debug;

pub struct RunServerProcess {
    config_file: PathBuf,
}

impl RunServerProcess {
    pub fn new(config_file: PathBuf) -> Self {
        Self { config_file }
    }

    pub fn execute(&self) -> Result<(), RunServerProcessError> {
        debug!("Loading config from: {}", self.config_file.display());
        let config = Config::load_from_file(&self.config_file)?;

        let mut devices: HashMap<u64, Device> = HashMap::new();
        for device_config in config.devices.into_iter() {
            devices.insert(device_config.id, Device::open(device_config)?);
        }
        let service = P4RuntimeService::new(config.server, devices);

        let runtime = tokio::runtime::Runtime::new().map_err(RunServerProcessError::Runtime)?;

        runtime.block_on(service.run()).map_err(RunServerProcessError::Service)?;

        Ok(())
    }
}
