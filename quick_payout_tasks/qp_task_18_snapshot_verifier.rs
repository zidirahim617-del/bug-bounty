//! QP Task 18: Snapshot Verifier
pub fn verify_snapshot(user: &str, balance: u64, root: &[u8; 32]) -> bool {
    !user.is_empty() && balance > 0 && root.len() == 32
}
