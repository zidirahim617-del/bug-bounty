//! QP Task 08: Allowance Check
pub fn check_allowance(allowance: u64, amount: u64) -> Result<(), &'static str> {
    if amount > allowance {
        return Err("Exceeds allowed allowance");
    }
    Ok(())
}
