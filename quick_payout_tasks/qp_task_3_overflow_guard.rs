//! Quick Payout Task 3: Checked Arithmetic Overflow Guard
//! Target: Algora / Polar 4-7 Day Bounties
//! Payout Wallet: 0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c

pub fn safe_add(a: u64, b: u64) -> Result<u64, &'static str> {
    a.checked_add(b).ok_or("Arithmetic overflow detected")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_guard() {
        assert_eq!(safe_add(100, 200), Ok(300));
        assert!(safe_add(u64::MAX, 1).is_err());
    }
}
