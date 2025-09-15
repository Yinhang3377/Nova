use rand::Rng;
use sha2::{ Digest, Sha256 };

/// Very small, non-cryptographic PoH-like digest generator used for tests.
/// Input: previous digest bytes and a counter. Output: 32-byte digest.
pub fn generate_poh(previous: &[u8], counter: u64) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(previous);
    hasher.update(counter.to_le_bytes());
    let result = hasher.finalize();
    result.to_vec()
}

/// Utility to produce an initial random seed digest for tests.
pub fn random_seed() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; 32];
    rng.fill(&mut buf);
    buf.to_vec()
}
