//! rocks_db adapter stub
//!
//! This is a small adapter that implements the `Storage` trait by delegating
//! to the in-memory `MemDb`. It serves as a placeholder so higher-level code
//! can be written against a `rocks_db` module while a real RocksDB-backed
//! implementation is developed.

use crate::storage::mem_db::MemDb;
use crate::storage::Storage;

#[derive(Clone, Default)]
pub struct RocksDbStub {
    inner: MemDb,
}

impl RocksDbStub {
    pub fn new() -> Self {
        Self {
            inner: MemDb::new(),
        }
    }
}

impl Storage for RocksDbStub {
    fn put(&self, key: Vec<u8>, value: Vec<u8>) {
        self.inner.put(key, value)
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.inner.get(key)
    }

    fn delete(&self, key: &[u8]) {
        self.inner.delete(key)
    }

    fn contains_key(&self, key: &[u8]) -> bool {
        self.inner.contains_key(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rocks_stub_put_get() {
        let db = RocksDbStub::new();
        let k = b"rkey".to_vec();
        let v = b"rval".to_vec();
        db.put(k.clone(), v.clone());
        assert_eq!(db.get(&k).as_deref(), Some(&v[..]));
        db.delete(&k);
        assert!(!db.contains_key(&k));
    }
}
