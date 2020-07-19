use crate::{KvsError, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::io::{BufReader, Read};
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;

const MAX_UNCOMPACTED_SIZE: u64 = 1024 * 1024;

#[derive(Debug, Deserialize, Serialize)]
enum Command {
    Set { key: String, value: String },
    Rm { key: String },
    Get { key: String },
}

/// the `KvStore` using a hashmap to store value in the memory
pub struct KvStore {
    map: HashMap<String, LogInFile>,
    command: Option<Command>,
    buffer: BufReader<std::fs::File>,
    position: u64,
    uncompacted_size: u64,
    path: PathBuf,
}

impl KvStore {
    /// This method used to create a KvStore
    pub fn new(buffer: BufReader<std::fs::File>, path: PathBuf) -> KvStore {
        KvStore {
            map: HashMap::new(),
            command: None,
            buffer,
            position: 0,
            uncompacted_size: 0,
            path,
        }
    }

    /// This method used to set a new key-value pair,
    /// It can also be used to update the value of a key
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // self.map.insert(key, value)
        // unimplemented!();
        let key_clone = key.clone();
        self.command = Some(Command::Set { key, value });
        let j = serde_json::to_string(&self.command)?;
        let len = (j.len() + "\n".len()) as u64;
        self.map
            .insert(key_clone, LogInFile::new(self.position, len));
        self.position += len;
        let mut f = self.buffer.get_ref();
        serde_json::to_writer(f, &self.command)?;
        // let mut f = OpenOptions::new().write(true).append(true).create(true).open("kvs-value.json")?;
        f.write(b"\n")?;
        self.uncompacted_size += len;
        if self.uncompacted_size > MAX_UNCOMPACTED_SIZE {
            self.compact()?;
        }
        Ok(())
    }

    /// This method used to get a value of the key in the Option.
    /// Key not been set will return None
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // self.map.get(&key).cloned()
        // unimplemented!();
        match self.map.get(&key) {
            None => Ok(None),
            Some(log) => {
                let reader = self.buffer.get_mut();
                reader.seek(SeekFrom::Start(log.offset))?;
                let cmd = reader.take(log.length);
                if let Command::Set { value, .. } = serde_json::from_reader(cmd)? {
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// This method used to remove a key-value pair.alloc
    pub fn remove(&mut self, key: String) -> Result<()> {
        // self.map.remove(&key);
        // unimplemented!();
        match self.map.get(&key) {
            None => return Err(KvsError::KeyNotFoundError),
            Some(_) => {
                // println!("{}", value);
                self.map.remove(&key);
                self.command = Some(Command::Rm { key });
                let j = serde_json::to_string(&self.command)?;
                let len = (j.len() + "\n".len()) as u64;
                self.position += len;
                self.uncompacted_size += len;
                let mut f = self.buffer.get_ref();
                serde_json::to_writer(f, &self.command)?;
                f.write(b"\n")?;
                if self.uncompacted_size > MAX_UNCOMPACTED_SIZE {
                    self.compact()?;
                }
            }
        }
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path: PathBuf = path.into();
        let path_clone = path.clone();
        path.push("kvs-data.json");
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(&path)?;
        let buffer = BufReader::new(f);
        let mut str_buffer = String::new();
        let mut kvstore = KvStore::new(buffer, path_clone);
        kvstore.buffer.read_to_string(&mut str_buffer)?;
        // println!("{}", buffer);
        for s in str_buffer.split("\n").collect::<Vec<&str>>() {
            if s.len() == 0 {
                continue;
            }
            let len = s.len() as u64;
            let c: Command = serde_json::from_str(s)?;
            match c {
                Command::Set { key, .. } => {
                    kvstore
                        .map
                        .insert(key, LogInFile::new(kvstore.position, len));
                }
                Command::Rm { key } => {
                    kvstore.map.remove(&key);
                }
                _ => (),
            }
            kvstore.position += len + "\n".len() as u64;
        }
        Ok(kvstore)
    }

    pub fn compact(&mut self) -> Result<()> {
        let mut path_from = self.path.clone();
        let mut path_to = self.path.clone();
        path_from.push("kvs-data-compact.json");
        path_to.push("kvs-data.json");
        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&path_from)?;
        let reader = self.buffer.get_mut();
        let mut new_offset: u64 = 0;
        for (_key, LogInFile { offset, length }) in self.map.iter_mut() {
            reader.seek(SeekFrom::Start(*offset))?;
            let mut cmd = reader.take(*length);
            std::io::copy(&mut cmd, &mut f)?;
            *offset = new_offset;
            // I still don't know why the compact test will add the split of mine twice.
            // f.write(b"\n")?;
            // new_offset += *length + "\n".len() as u64;
            new_offset += *length;
        }
        fs::rename(path_from, path_to)?;
        self.uncompacted_size = 0;
        self.position = f.seek(SeekFrom::End(0))?;
        Ok(())
    }
}

struct LogInFile {
    offset: u64,
    length: u64,
}

impl LogInFile {
    fn new(offset: u64, length: u64) -> LogInFile {
        LogInFile { offset, length }
    }
}
