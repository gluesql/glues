use crate::{
    event::Event,
    state::{note_tree::NoteTreeState, State},
    Error, Glues, Result,
};

pub struct EntryState;

impl EntryState {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<()> {
        match (&glues.state, event) {
            (State::Entry(_), Event::Initialize) => {
                glues.state = State::NoteTree(NoteTreeState::new(glues).await?);
                Ok(())
            }
            _ => Err(Error::Wip("todo: EntryState::consume".to_owned())),
        }
    }

    pub fn describe(&self) -> String {
        "Entry".to_owned()
    }
}
