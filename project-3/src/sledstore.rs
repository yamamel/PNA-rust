use sled;
use crate::KvsEngine;
use std::path::PathBuf;
use crate::Result;

pub struct SledStore {
    sled: sled::Db,
}

impl SledStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<SledStore> {
        let mut path: PathBuf = path.into();
        path.push("seld-data");
        Ok(SledStore {
            sled: sled::open(path)?,
        })
    }
}

impl KvsEngine for SledStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.sled.insert(key.as_bytes(), value.as_bytes())?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        let t = self.sled.get(key.as_bytes())?;
        Ok(t.map(|v| String::from_utf8(v.to_vec()).expect("Found invalid utf-8")))
    }

    fn remove(&mut self, key: String) -> Result<()> {
        self.sled.remove(key.as_bytes())?;
        Ok(())
    }
}