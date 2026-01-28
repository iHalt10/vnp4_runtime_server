use thiserror::Error;

#[derive(Debug, Error)]
pub enum MmioError {
    #[error("Failed to open '{path}'")]
    Open {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to mmap")]
    Mmap,
    #[error("Failed to munmap")]
    Munmap,
    #[error("Address {address:#x} is out of bounds (region size: {size:#x})")]
    OutOfBounds { address: usize, size: usize },
    #[error("Address {address:#x} is not aligned to {alignment} bytes")]
    UnalignedAccess { address: usize, alignment: usize },
    #[error("Region already closed")]
    AlreadyClosed,
}
