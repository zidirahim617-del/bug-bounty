//! QP Task 12: Gas Estimator
pub fn estimate_gas(instructions_count: usize, base_fee: u64) -> u64 {
    (instructions_count as u64).saturating_mul(21000).saturating_add(base_fee)
}
