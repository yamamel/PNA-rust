#![deny(missing_docs)]
//! this crate is use to store key-value pair
pub use error::{KvsError, Result};
pub use kv::KvStore;

mod error;
mod kv;
