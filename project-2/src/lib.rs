// #![deny(missing_docs)]
//! this crate is use to store key-value pair
pub use kv::KvStore;
pub use error::{KvsError, Result};

mod kv;
mod error;