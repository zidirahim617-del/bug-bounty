//! QP Task 14: Voting Power
pub fn calculate_voting_power(staked_amount: u64, multiplier: u64) -> u64 {
    staked_amount.saturating_mul(multiplier)
}
