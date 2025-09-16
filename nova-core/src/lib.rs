//! nova-core: core primitives for the Nova chain (skeleton)

pub mod bridge;
pub mod consensus;
pub mod data_model;
pub mod governance;
pub mod network;
pub mod storage;
pub mod vm;
// Inline a small wallet module to provide a minimal demo API without creating
// additional top-level files. This keeps the existing `wallet` submodules
// available via `crate::wallet::...` if desired; for now we provide a simple
// `send` function used by the CLI demo.
pub mod wallet {
    pub mod hd_address;
    pub mod key_manager;
    pub mod mempool;

    pub fn send(amount: u64) {
        println!("钱包发送 币: {}", amount);
        crate::bridge::lock(amount);
    }
}

pub use consensus::poh;

// ...existing code...
