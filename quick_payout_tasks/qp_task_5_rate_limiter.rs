//! Quick Payout Task 5: Simple Timestamp Rate Limiter
//! Target: Algora / Polar 4-7 Day Bounties
//! Payout Wallet: 0x9be208a4dC48974Bb857dc543F5B2a7654d73B8c

pub struct RateLimiter {
    last_action_timestamp: u64,
    cooldown_seconds: u64,
}

impl RateLimiter {
    pub fn new(cooldown_seconds: u64) -> Self {
        Self {
            last_action_timestamp: 0,
            cooldown_seconds,
        }
    }

    pub fn try_action(&mut self, current_timestamp: u64) -> Result<(), &'static str> {
        if current_timestamp < self.last_action_timestamp + self.cooldown_seconds {
            return Err("Rate limit exceeded: please wait for cooldown");
        }
        self.last_action_timestamp = current_timestamp;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(60);
        assert!(limiter.try_action(100).is_ok());
        assert!(limiter.try_action(120).is_err()); // within 60s cooldown
        assert!(limiter.try_action(165).is_ok()); // after cooldown
    }
}
