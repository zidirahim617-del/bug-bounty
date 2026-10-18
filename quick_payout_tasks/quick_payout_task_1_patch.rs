//! Quick Payout Task 1: Rust Safe Result Unwrapping & Payout Claim Kit
//! Target: Algora / Polar 4-6 Day Bounties
//! Payout Wallet: 0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c

pub fn safe_parse_config(input: Option<&str>) -> Result<String, &'static str> {
    input.map(|s| s.to_string()).ok_or("Config missing: prevented unwrap panic")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_parse() {
        assert!(safe_parse_config(Some("valid")).is_ok());
        assert!(safe_parse_config(None).is_err());
    }
}
