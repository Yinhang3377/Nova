use crate::storage::Storage;

/// Tiny backend factory for tests and local runs.
/// Returns a boxed Storage trait object for a given backend name.
pub fn open_backend(name: &str) -> Box<dyn Storage> {
    match name {
        "mem" => Box::new(crate::storage::mem_db::MemDb::new()),
        "rocks" => Box::new(crate::storage::rocks_db::RocksDbStub::new()),
        "sled" => Box::new(crate::storage::sled_db::SledDbStub::new()),
        _ => Box::new(crate::storage::mem_db::MemDb::new()),
    }
}
