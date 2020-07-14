use std::collections::HashMap;

#[allow(dead_code)]
/// the `KvStore` using a hashmap to store value in the memory
pub struct KvStore {
    map: HashMap<String, String>,
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
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
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
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
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
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
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
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
