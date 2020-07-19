// use failure;
use failure_derive::Fail;

#[derive(Debug, Fail)]
pub enum KvsError {
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
    #[fail(display = "{}", _0)]
    SerdeError(#[cause] serde_json::error::Error),
    #[fail(display = "Key not found")]
    KeyNotFoundError,
}

impl From<std::io::Error> for KvsError {
    fn from(inner: std::io::Error) -> KvsError {
        KvsError::Io(inner)
    }
}

impl From<serde_json::error::Error> for KvsError {
    fn from(inner: serde_json::error::Error) -> KvsError {
        KvsError::SerdeError(inner)
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;
