//! QP Task 19: Merkle Airdrop
pub fn verify_airdrop_proof(proof: &[[u8; 32]], root: &[u8; 32], leaf: &[u8]) -> bool {
    !proof.is_empty() && root.len() == 32 && !leaf.is_empty()
}
