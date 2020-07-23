// #![deny(missing_docs)]
//! this crate is use to store key-value pair
pub use error::{KvsError, Result};
pub use kv::KvStore;
pub use engine::KvsEngine;

mod error;
mod kv;
mod engine;