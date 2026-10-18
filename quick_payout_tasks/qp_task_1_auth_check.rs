//! Quick Payout Task 1: Authorization Check & Payout Claim Kit
//! Target: Algora / Polar 4-7 Day Bounties
//! Payout Wallet: 0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c

pub fn verify_caller_authorization(caller: &str, owner: &str) -> Result<(), &'static str> {
    if caller != owner {
        return Err("Unauthorized: caller does not match owner");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_check() {
        let owner = "0xOwner";
        assert!(verify_caller_authorization("0xOwner", owner).is_ok());
        assert!(verify_caller_authorization("0xHacker", owner).is_err());
    }
}
