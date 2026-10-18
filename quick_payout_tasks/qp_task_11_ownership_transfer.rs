//! QP Task 11: Ownership Transfer
pub struct Ownership {
    pub owner: String,
    pub pending_owner: Option<String>,
}
impl Ownership {
    pub fn new(owner: &str) -> Self { Self { owner: owner.to_string(), pending_owner: None } }
    pub fn transfer(&mut self, new_owner: &str) { self.pending_owner = Some(new_owner.to_string()); }
    pub fn claim(&mut self, caller: &str) -> Result<(), &'static str> {
        let pending = self.pending_owner.as_ref().ok_or("No pending owner")?;
        if pending != caller { return Err("Caller is not pending owner"); }
        self.owner = pending.clone();
        self.pending_owner = None;
        Ok(())
    }
}
