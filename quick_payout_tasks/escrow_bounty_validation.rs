//! Rust Soroban-Style Escrow Bounty Validation & Safety Module
//! Designed for quick-payout 4-7 day bounty tasks (Soroban / Rust smart contracts).

pub struct EscrowBounty {
    pub id: u64,
    pub creator: [u8; 32],
    pub amount: u64,
    pub deadline: u64,
    pub is_funded: bool,
    pub is_refunded: bool,
}

impl EscrowBounty {
    pub fn new(id: u64, creator: [u8; 32], amount: u64, deadline: u64) -> Self {
        Self {
            id,
            creator,
            amount,
            deadline,
            is_funded: true,
            is_refunded: false,
        }
    }

    /// Reopens an expired bounty securely with strict auth and future deadline checks
    pub fn reopen(&mut self, caller: [u8; 32], new_deadline: u64, current_timestamp: u64) -> Result<(), &'static str> {
        if self.creator != caller {
            return Err("Unauthorized: only the bounty creator can reopen");
        }
        if self.is_refunded {
            return Err("Rejected: funds were already refunded");
        }
        if new_deadline <= current_timestamp {
            return Err("Rejected: new deadline must be in the future");
        }
        
        self.deadline = new_deadline;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reopen_bounty_success() {
        let creator = [1u8; 32];
        let mut bounty = EscrowBounty::new(1, creator, 1000, 100);
        assert!(bounty.reopen(creator, 200, 150).is_ok());
        assert_eq!(bounty.deadline, 200);
    }

    #[test]
    fn test_reopen_bounty_unauthorized() {
        let creator = [1u8; 32];
        let stranger = [2u8; 32];
        let mut bounty = EscrowBounty::new(1, creator, 1000, 100);
        assert!(bounty.reopen(stranger, 200, 150).is_err());
    }
}
