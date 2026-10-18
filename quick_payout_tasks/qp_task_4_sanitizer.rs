//! Quick Payout Task 4: Input Sanitization & String Cleanup
//! Target: Algora / Polar 4-7 Day Bounties
//! Payout Wallet: 0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c

pub fn sanitize_string(input: &str) -> String {
    input.trim().chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitizer() {
        assert_eq!(sanitize_string("  user_123!@# "), "user_123");
    }
}
