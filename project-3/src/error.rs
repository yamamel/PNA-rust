// use failure;
use failure_derive::Fail;

/// Error type for kvs
#[derive(Debug, Fail)]
pub enum KvsError {
    /// caused by IO error
    #[fail(display = "{}", _0)]
    IoError(#[cause] std::io::Error),
    /// caused by serde error
    #[fail(display = "{}", _0)]
    SerdeError(#[cause] serde_json::error::Error),
    /// caused by key not found
    #[fail(display = "Key not found")]
    KeyNotFoundError,
}

impl From<std::io::Error> for KvsError {
    fn from(inner: std::io::Error) -> KvsError {
        KvsError::IoError(inner)
    }
}

impl From<serde_json::error::Error> for KvsError {
    fn from(inner: serde_json::error::Error) -> KvsError {
        KvsError::SerdeError(inner)
    }
}
/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;
