//! Work 6: State Machine
#[derive(Clone, Copy, PartialEq)]
pub enum State { Created, Funded, Completed }

pub fn transition(current: State, next: State) -> Result<State, &'static str> {
    match (current, next) {
        (State::Created, State::Funded) => Ok(State::Funded),
        (State::Funded, State::Completed) => Ok(State::Completed),
        _ => Err("Invalid state transition"),
    }
}
