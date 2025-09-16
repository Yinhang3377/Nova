//! bridge skeleton

pub mod zkg;

// ...existing code...

// Minimal public helper used by the CLI demo.
pub fn lock(amount: u64) {
    println!("桥锁住 币: {}", amount);
    crate::consensus::propose();
}
