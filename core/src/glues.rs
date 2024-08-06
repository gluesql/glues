use crate::{
    db::Db,
    state::{EntryState, State},
    Event, Result, Transition,
};

pub struct Glues {
    pub db: Option<Db>,
    pub state: State,
}

impl Glues {
    pub async fn new() -> Self {
        Self {
            db: None,
            state: EntryState.into(),
        }
    }

    pub async fn dispatch(&mut self, event: Event) -> Result<Transition> {
        State::consume(self, event).await
    }
}
