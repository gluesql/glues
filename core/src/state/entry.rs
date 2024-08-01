use crate::{
    state::notebook::NotebookState, EntryEvent, EntryTransition, Error, Event, Glues, Result,
};

pub struct EntryState;

impl EntryState {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<EntryTransition> {
        match event {
            Event::Entry(EntryEvent::Initialize) => {
                glues.state = NotebookState::new(glues).await?.into();

                Ok(EntryTransition::Initialize)
            }
            Event::Key(_) => Ok(EntryTransition::Inedible(event)),
            _ => Err(Error::Wip("todo: EntryState::consume".to_owned())),
        }
    }

    pub fn describe(&self) -> Result<String> {
        Ok("Entry".to_owned())
    }

    pub fn shortcuts(&self) -> Vec<&str> {
        vec![":)"]
    }
}
