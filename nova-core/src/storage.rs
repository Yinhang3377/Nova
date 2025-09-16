//! storage skeleton
//! storage skeleton

pub mod factory;
pub mod mem_db;
pub mod rocks_db;
pub mod sled_db;
pub mod state_shards;

/// Generic storage trait for simple key/value backends used in tests and later substitution.
/// Keys and values are byte vectors to keep the interface low-level and flexible.
pub trait Storage: Send + Sync {
    /// Put a value into storage. Implementations should handle concurrency internally.
    fn put(&self, key: Vec<u8>, value: Vec<u8>);

    /// Get a value by key. Returns None if key is not present.
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Delete a key from storage.
    fn delete(&self, key: &[u8]);

    /// Check whether a key exists in storage.
    fn contains_key(&self, key: &[u8]) -> bool;
}
