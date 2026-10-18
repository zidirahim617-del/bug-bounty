//! Work 3: Timelock Escrow
pub fn can_withdraw(release_time: u64, current_time: u64) -> bool {
    current_time >= release_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timelock() {
        assert!(can_withdraw(100, 150));
        assert!(!can_withdraw(200, 150));
    }
}
