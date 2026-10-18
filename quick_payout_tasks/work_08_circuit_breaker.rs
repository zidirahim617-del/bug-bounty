//! Work 8: Circuit Breaker
pub struct CircuitBreaker {
    pub tripped: bool,
}
impl CircuitBreaker {
    pub fn new() -> Self { Self { tripped: false } }
    pub fn check(&self) -> Result<(), &'static str> {
        if self.tripped { Err("Circuit breaker tripped") } else { Ok(()) }
    }
}
