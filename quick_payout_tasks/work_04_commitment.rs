//! Work 4: Hash Commitment
use sha2::{Sha256, Digest};

pub fn verify_commitment(secret: &[u8], salt: &[u8], expected_hash: &[u8; 32]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(secret);
    hasher.update(salt);
    let result = hasher.finalize();
    result.as_slice() == expected_hash
}
