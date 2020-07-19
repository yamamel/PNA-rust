use std::collections::HashMap;
use std::path::PathBuf;
use crate::{Result, KvsError};
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Write;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
enum Command {
    Set { key: String, value: String},
    Rm { key: String },
    Get { key: String },
}

/// the `KvStore` using a hashmap to store value in the memory
pub struct KvStore {
    map: HashMap<String, String>,
    command: Option<Command>,
    buffer: std::fs::File,
}

impl KvStore {
    /// This method used to create a KvStore
    ///
    /// # Example
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut _kvstore = KvStore::new();
    /// ```
    pub fn new(buffer: std::fs::File) -> KvStore {
        KvStore {
            map: HashMap::new(),
            command: None,
            buffer,
        }
    }

    /// This method used to set a new key-value pair,
    /// It can also be used to update the value of a key
    ///
    /// # Example
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvstore = KvStore::new();
    /// kvstore.set("key01".to_owned(), "value01".to_owned());
    /// assert_eq!(kvstore.get("key01".to_owned()), Some("value01".to_owned()));
    /// kvstore.set("key01".to_owned(), "value02".to_owned());
    /// assert_eq!(kvstore.get("key01".to_owned()), Some("value02".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // self.map.insert(key, value)
        // unimplemented!();

        self.map.insert(key.clone(), value.clone());
        self.command = Some(Command::Set {key, value});
        serde_json::to_writer(&self.buffer, &self.command)?;
        // let mut f = OpenOptions::new().write(true).append(true).create(true).open("kvs-value.json")?;
        self.buffer.write(b"\n")?;
        Ok(())
    }

    /// This method used to get a value of the key in the Option.
    /// Key not been set will return None
    ///
    /// # Example
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvstore = KvStore::new();
    /// kvstore.set("key01".to_owned(), "value01".to_owned());
    /// assert_eq!(kvstore.get("key01".to_owned()), Some("value01".to_owned()));
    /// assert_eq!(kvstore.get("key02".to_owned()), None);
    /// ```
    pub fn get(&self, key: String) -> Result<Option<String>> {
        // self.map.get(&key).cloned()
        // unimplemented!();
        match self.map.get(&key) {
            None => Ok(None),
            Some(value) => {
                Ok(Some(value.clone()))
            }
        }
    }

    /// This method used to remove a key-value pair.alloc
    ///
    /// # Example
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvstore = KvStore::new();
    /// kvstore.set("key01".to_owned(), "value01".to_owned());
    /// assert_eq!(kvstore.get("key01".to_owned()), Some("value01".to_owned()));
    /// kvstore.remove("key01".to_owned());
    /// assert_eq!(kvstore.get("key01".to_owned()), None);
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        // self.map.remove(&key);
        // unimplemented!();
        match self.map.get(&key) {
            None => return Err(KvsError::KeyNotFoundError),
            Some(_) => {
                // println!("{}", value);
                self.map.remove(&key);
                self.command = Some(Command::Rm {key});
                serde_json::to_writer(&self.buffer, &self.command)?;
                self.buffer.write(b"\n")?;
            }
        }
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path: PathBuf = path.into();
        path.push("kvs-value.json");
        let f = OpenOptions::new().read(true).write(true).append(true).create(true).open(path)?;
        let mut str_buffer = String::new();
        let mut kvstore = KvStore::new(f);
        kvstore.buffer.read_to_string(&mut str_buffer)?;
        // println!("{}", buffer);
        for s in str_buffer.split("\n").collect::<Vec<&str>>() {
            if s.len() == 0 {
                continue;
            }
            let c: Command = serde_json::from_str(s)?;
            match c {
                Command::Set { key, value } => {
                    kvstore.map.insert(key, value);
                }
                Command::Rm { key } => {
                    kvstore.map.remove(&key);
                }
                _ => ()
            }
        }
        Ok(kvstore)
    }
}
