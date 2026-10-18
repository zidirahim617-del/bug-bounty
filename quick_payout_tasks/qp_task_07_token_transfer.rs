//! QP Task 07: Token Transfer
pub fn transfer_tokens(balance: u64, amount: u64) -> Result<u64, &'static str> {
    balance.checked_sub(amount).ok_or("Insufficient balance for transfer")
}
