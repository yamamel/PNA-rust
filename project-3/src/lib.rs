// #![deny(missing_docs)]
//! this crate is use to store key-value pair
pub use error::{KvsError, Result};
pub use kv::{KvStore, Command};
pub use engine::KvsEngine;
pub use sledstore::SledStore;

mod error;
mod kv;
mod engine;
mod sledstore;