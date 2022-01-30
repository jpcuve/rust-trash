#[derive(Debug)]
pub enum State {
    StateA, StateB, StateC,
}

impl State {
    pub fn transition(&self) -> State {
        match &self {
            State::StateA => State::StateB,
            State::StateB => State::StateC,
            State::StateC => State::StateA,
        }
    }
}