mod entry;
mod note_tree;

use crate::{event::Event, Glues, Result};

pub use {entry::EntryState, note_tree::NoteTreeState};

pub enum State {
    Entry(EntryState),
    NoteTree(note_tree::NoteTreeState),
}

impl State {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<()> {
        match event {
            Event::Initialize => EntryState::consume(glues, event).await,
            _ => NoteTreeState::consume(glues, event).await,
        }
    }

    pub fn describe(&self) -> String {
        match self {
            State::Entry(state) => state.describe(),
            State::NoteTree(state) => state.describe(),
        }
    }
}
