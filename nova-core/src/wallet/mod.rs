//! wallet module (consolidated)

pub mod hd_address;
pub mod key_manager;
pub mod mempool;

// Minimal public helper used by the CLI demo. Keeps existing module surface intact.
pub fn send(amount: u64) {
    println!("钱包发送 币: {}", amount);
    crate::bridge::lock(amount);
}
