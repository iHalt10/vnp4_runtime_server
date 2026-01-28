use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read '{path}'")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to serde_yaml")]
    SerdeYaml(#[from] serde_yaml::Error),
}

#[derive(Debug, Error)]
pub enum RunServerProcessError {
    #[error("Failed to config")]
    Config(#[from] ConfigError),

    #[error("Failed to device")]
    Device(#[from] crate::target::models::DeviceError),

    #[error("Failed to runtime")]
    Runtime(#[source] std::io::Error),

    #[error("Failed to service")]
    Service(Box<dyn std::error::Error>),
}
