//! QP Task 15: Stake Calculator
pub fn calculate_rewards(stake: u64, rate_bps: u64, duration_days: u64) -> u64 {
    (stake as u128 * rate_bps as u128 * duration_days as u128 / (10000 * 365)) as u64
}
