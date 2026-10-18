//! QP Task 16: Reward Distributor
pub fn distribute_rewards(total_reward: u64, shares: &[u64]) -> Vec<u64> {
    let total_shares: u64 = shares.iter().sum();
    if total_shares == 0 { return vec![0; shares.len()]; }
    shares.iter().map(|&s| (total_reward as u128 * s as u128 / total_shares as u128) as u64).collect()
}
