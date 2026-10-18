//! Cryptographic Merkle Tree Inclusion Proof Verifier
//! Target: L2 Cross-Chain Bridges & Consensus Verification

use sha2::{Sha256, Digest};

pub fn verify_merkle_proof(leaf: &[u8], proof: &[[u8; 32]], root: &[u8; 32], index: usize) -> bool {
    let mut computed_hash = Sha256::digest(leaf);
    let mut current_index = index;

    for sibling in proof {
        let mut hasher = Sha256::new();
        if current_index % 2 == 0 {
            hasher.update(&computed_hash);
            hasher.update(sibling);
        } else {
            hasher.update(sibling);
            hasher.update(&computed_hash);
        }
        computed_hash.copy_from_slice(&hasher.finalize());
        current_index /= 2;
    }

    computed_hash.as_slice() == root
}
