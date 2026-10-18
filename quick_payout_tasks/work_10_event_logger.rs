//! Work 10: Event Logger
pub struct EventLog {
    pub events: Vec<String>,
}
impl EventLog {
    pub fn new() -> Self { Self { events: Vec::new() } }
    pub fn log(&mut self, msg: &str) { self.events.push(msg.to_string()); }
}
