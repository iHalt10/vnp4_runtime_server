use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("Failed to read '{path}'")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write'{path}'")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to serde_json")]
    SerdeJson(#[from] serde_json::Error),
}
