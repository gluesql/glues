use crate::{
    Event, Result, Transition,
    backend::BackendBox,
    state::{EntryState, State},
};

pub struct Glues {
    pub db: Option<BackendBox>,
    pub state: State,
}

impl Default for Glues {
    fn default() -> Self {
        Self::new()
    }
}

impl Glues {
    pub fn new() -> Self {
        Self {
            db: None,
            state: EntryState.into(),
        }
    }

    pub async fn dispatch(&mut self, event: Event) -> Result<Transition> {
        State::consume(self, event).await
    }
}
