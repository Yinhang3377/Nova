use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A simple thread-safe in-memory key-value store used for testing.
#[derive(Clone, Default)]
pub struct MemDb {
    inner: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl MemDb {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn put(&self, key: Vec<u8>, value: Vec<u8>) {
        let mut map = self.inner.write().expect("lock poisoned");
        map.insert(key, value);
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let map = self.inner.read().expect("lock poisoned");
        map.get(key).cloned()
    }

    pub fn delete(&self, key: &[u8]) {
        let mut map = self.inner.write().expect("lock poisoned");
        map.remove(key);
    }

    pub fn contains_key(&self, key: &[u8]) -> bool {
        let map = self.inner.read().expect("lock poisoned");
        map.contains_key(key)
    }
}

use crate::storage::Storage;

impl Storage for MemDb {
    fn put(&self, key: Vec<u8>, value: Vec<u8>) {
        self.put(key, value)
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.get(key)
    }

    fn delete(&self, key: &[u8]) {
        self.delete(key)
    }

    fn contains_key(&self, key: &[u8]) -> bool {
        self.contains_key(key)
    }
}

#[cfg(test)]
mod tests {
    use super::MemDb;

    #[test]
    fn basic_put_get_delete() {
        let db = MemDb::new();
        let k = b"key".to_vec();
        let v = b"value".to_vec();

        assert!(!db.contains_key(&k));
        db.put(k.clone(), v.clone());
        assert!(db.contains_key(&k));
        assert_eq!(db.get(&k).as_deref(), Some(&v[..]));

        db.delete(&k);
        assert!(!db.contains_key(&k));
        assert_eq!(db.get(&k), None);
    }

    #[test]
    fn concurrent_puts() {
        use std::thread;

        let db = MemDb::new();
        let mut handles = vec![];

        for i in 0..8 {
            let dbc = db.clone();
            handles.push(thread::spawn(move || {
                let k = format!("k{}", i).into_bytes();
                let v = format!("v{}", i).into_bytes();
                dbc.put(k, v);
            }));
        }

        for h in handles {
            h.join().expect("thread panicked");
        }

        // check some keys exist
        assert!(db.contains_key(b"k0"));
        assert!(db.contains_key(b"k7"));
    }
}
