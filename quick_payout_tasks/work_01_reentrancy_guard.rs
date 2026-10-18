//! Work 1: Reentrancy Guard Module
pub struct ReentrancyGuard {
    locked: bool,
}

impl ReentrancyGuard {
    pub fn new() -> Self {
        Self { locked: false }
    }

    pub fn enter(&mut self) -> Result<(), &'static str> {
        if self.locked {
            return Err("Reentrant call detected");
        }
        self.locked = true;
        Ok(())
    }

    pub fn exit(&mut self) {
        self.locked = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard() {
        let mut g = ReentrancyGuard::new();
        assert!(g.enter().is_ok());
        assert!(g.enter().is_err());
        g.exit();
        assert!(g.enter().is_ok());
    }
}
