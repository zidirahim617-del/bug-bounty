//! Algora & Polar Profitability Calculator
//! Computes expected ROI per hour for quick-payout GitHub bounty tasks.

pub struct BountyTask {
    pub title: &'static str,
    pub reward_usdc: f64,
    pub estimated_hours: f64,
}

impl BountyTask {
    pub fn roi_per_hour(&self) -> f64 {
        self.reward_usdc / self.estimated_hours
    }
}

pub fn evaluate_top_bounties() -> Vec<(String, f64)> {
    let tasks = vec![
        BountyTask { title: "Fix unwrap panic in consensus module", reward_usdc: 75.0, estimated_hours: 1.5 },
        BountyTask { title: "Add missing invariant test for vault shares", reward_usdc: 150.0, estimated_hours: 2.0 },
        BountyTask { title: "Optimize gas in storage layout", reward_usdc: 250.0, estimated_hours: 3.0 },
    ];

    tasks.iter().map(|t| (t.title.to_string(), t.roi_per_hour())).collect()
}
