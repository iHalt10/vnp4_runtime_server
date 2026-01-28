use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateTargetConfigProcessError {
    #[error("Failed to libloading")]
    Libloading(#[from] libloading::Error),

    #[error("Failed to json")]
    Json(#[from] crate::utils::serde::JsonError),
}
