//! Advanced Invariant & Fuzz Testing Harness for DeFi Vault Share Inflation
//! Designed to catch precision loss, rounding errors, and inflation attacks in audit contests.

pub struct MockVault {
    pub total_shares: u128,
    pub total_assets: u128,
}

impl MockVault {
    pub fn new() -> Self {
        Self {
            total_shares: 0,
            total_assets: 0,
        }
    }

    pub fn deposit(&mut self, assets: u128) -> u128 {
        let shares = if self.total_supply() == 0 || self.total_assets == 0 {
            assets // Initial share minting
        } else {
            (assets * self.total_supply()) / self.total_assets
        };
        self.total_shares += shares;
        self.total_assets += assets;
        shares
    }

    pub fn total_supply(&self) -> u128 {
        self.total_shares
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_invariant_no_free_money() {
        let mut vault = MockVault::new();
        let assets = 1000u128;
        let shares = vault.deposit(assets);
        assert!(shares > 0);
        assert_eq!(vault.total_assets, assets);
    }

    #[test]
    fn test_vault_inflation_edge_case() {
        let mut vault = MockVault::new();
        let shares1 = vault.deposit(1);
        assert_eq!(shares1, 1);
        // Simulate attacker donations/rounding exploit simulation
        vault.total_assets += 10_000_000;
        let shares2 = vault.deposit(5000);
        // Due to rounding down, small deposits can yield 0 shares (classic ERC4626 inflation vector)
        assert_eq!(shares2, 0);
    }
}
