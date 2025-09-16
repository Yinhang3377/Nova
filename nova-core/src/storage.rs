//! storage skeleton

pub mod rocks_db;
pub mod sled_db;
pub mod state_shards;
pub mod mem_db;
pub mod factory;

/// Generic storage trait for simple key/value backends used in tests and later substitution.
/// Keys and values are byte vectors to keep the interface low-level and flexible.
pub trait Storage: Send + Sync {
    fn put(&self, key: Vec<u8>, value: Vec<u8>);
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn delete(&self, key: &[u8]);
    fn contains_key(&self, key: &[u8]) -> bool;
}

// ...existing code...
