use crate::{state::note_tree::NoteTreeState, Error, Event, Glues, Result, Transition};

pub struct EntryState;

impl EntryState {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<Transition> {
        match event {
            Event::Initialize => {
                glues.state = NoteTreeState::new(glues).await?.into();

                Ok(Transition::None)
            }
            Event::Key(_) => Ok(Transition::None),
            _ => Err(Error::Wip("todo: EntryState::consume".to_owned())),
        }
    }

    pub fn describe(&self) -> String {
        "Entry".to_owned()
    }

    pub fn shortcuts(&self) -> Vec<String> {
        vec![":)".to_owned()]
    }
}
