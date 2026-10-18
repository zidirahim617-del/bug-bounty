//! QP Task 20: Multi-Sig Mock
pub fn verify_multisig(signatures_count: usize, threshold: usize) -> bool {
    signatures_count >= threshold
}
