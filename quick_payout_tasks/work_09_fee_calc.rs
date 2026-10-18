//! Work 9: Fee Calculation
pub fn calculate_fee(amount: u64, fee_bps: u64) -> u64 {
    (amount as u128 * fee_bps as u128 / 10000) as u64
}
