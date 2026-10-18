//! Quick Payout Task 2: Safe Array Bounds Verification
//! Target: Algora / Polar 4-7 Day Bounties
//! Payout Wallet: 0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c

pub fn safe_get_index(data: &[u64], index: usize) -> Result<u64, &'static str> {
    if index >= data.len() {
        return Err("Index out of bounds");
    }
    Ok(data[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bound_check() {
        let arr = vec![10, 20, 30];
        assert_eq!(safe_get_index(&arr, 1), Ok(20));
        assert!(safe_get_index(&arr, 5).is_err());
    }
}
