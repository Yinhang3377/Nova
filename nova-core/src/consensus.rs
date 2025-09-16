//! consensus skeleton

pub mod bft_quorum;
pub mod driftrd;
pub mod poh;

// ...existing code...

// Minimal public helper used by the CLI demo.
pub fn propose() {
    println!("共识打包区块");
}
