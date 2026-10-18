//! Quick Payout Task 3: Rust Memory Alignment & Safety Checker
//! Target: Algora / Polar 4-6 Day Bounties
//! Payout Wallet: 0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c

pub fn check_alignment(addr: usize, align: usize) -> bool {
    addr % align == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_alignment() {
        assert!(check_alignment(64, 8));
        assert!(!check_alignment(65, 8));
    }
}
