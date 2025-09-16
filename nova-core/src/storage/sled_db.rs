//! sled_db adapter stub
//!
//! Placeholder adapter that delegates to the in-memory MemDb. Replace with
//! a real sled-backed implementation when desired.

use crate::storage::mem_db::MemDb;
use crate::storage::Storage;

#[derive(Clone)]
pub struct SledDbStub {
    inner: MemDb,
}

impl SledDbStub {
    pub fn new() -> Self {
        Self {
            inner: MemDb::new(),
        }
    }
}

impl Storage for SledDbStub {
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
    fn sled_stub_put_get() {
        let db = SledDbStub::new();
        let k = b"skey".to_vec();
        let v = b"sval".to_vec();
        db.put(k.clone(), v.clone());
        assert_eq!(db.get(&k).as_deref(), Some(&v[..]));
        db.delete(&k);
        assert!(!db.contains_key(&k));
    }
}
