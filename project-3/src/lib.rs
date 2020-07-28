// #![deny(missing_docs)]
//! this crate is use to store key-value pair
pub use engine::KvsEngine;
pub use error::{KvsError, Result};
pub use kv::{Command, KvStore};
pub use sledstore::SledStore;

mod engine;
mod error;
mod kv;
mod sledstore;
