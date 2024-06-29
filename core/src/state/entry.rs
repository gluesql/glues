use crate::{
    state::{note_tree::NoteTreeState, State},
    Error, Event, Glues, Result,
};

pub struct EntryState;

impl EntryState {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<()> {
        match (&glues.state, event) {
            (State::EntryState(_), Event::Initialize) => {
                glues.state = NoteTreeState::new(glues).await?.into();
                Ok(())
            }
            _ => Err(Error::Wip("todo: EntryState::consume".to_owned())),
        }
    }

    pub fn describe(&self) -> String {
        "Entry".to_owned()
    }
}
