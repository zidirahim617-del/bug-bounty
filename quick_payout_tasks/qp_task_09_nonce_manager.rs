//! QP Task 09: Nonce Manager
pub struct NonceManager {
    expected_nonce: u64,
}
impl NonceManager {
    pub fn new(start: u64) -> Self { Self { expected_nonce: start } }
    pub fn verify_and_increment(&mut self, nonce: u64) -> Result<(), &'static str> {
        if nonce != self.expected_nonce {
            return Err("Invalid nonce");
        }
        self.expected_nonce += 1;
        Ok(())
    }
}
