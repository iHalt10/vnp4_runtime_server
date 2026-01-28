use crate::target::driver::XilVitisNetP4ReturnType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TableError {
    #[error("Failed Driver: {name}({code})")]
    Driver { name: String, code: XilVitisNetP4ReturnType },

    #[error("Not supported")]
    NotSupported,

    #[error("Not found action")]
    NotFoundAction,

    #[error("Not found key")]
    NotFoundKey,
}

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Failed to mmio")]
    Mmio(#[from] crate::utils::mmio::MmioError),

    #[error("Failed to driver [{name}({code})]")]
    Driver { name: String, code: XilVitisNetP4ReturnType },

    #[error("Failed to json")]
    Json(#[from] crate::utils::serde::JsonError),

    #[error("Failed to json")]
    Table(#[from] TableError),
}
