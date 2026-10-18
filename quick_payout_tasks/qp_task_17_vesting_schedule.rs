//! QP Task 17: Vesting Schedule
pub fn vested_amount(total_allocation: u64, start: u64, cliff: u64, duration: u64, now: u64) -> u64 {
    if now < start + cliff { return 0; }
    if now >= start + duration { return total_allocation; }
    let elapsed = now - start;
    (total_allocation as u128 * elapsed as u128 / duration as u128) as u64
}
