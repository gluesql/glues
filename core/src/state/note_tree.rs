use crate::{
    event::Event,
    state::State,
    types::{DirectoryId, NoteId},
    Error, Glues, Result,
};

pub struct NoteTreeState {
    selected: Selected,
}

pub enum Selected {
    Note(NoteId),
    Directory(DirectoryId),
}

impl NoteTreeState {
    pub async fn new(glues: &mut Glues) -> Result<Self> {
        // let directory = glues.fetch_directory(glues.root_id).await?;
        let directory_id = glues.root_id.clone();

        Ok(NoteTreeState {
            selected: Selected::Directory(directory_id),
        })
    }

    pub async fn consume(glues: &mut Glues, event: Event) -> Result<()> {
        match (&mut glues.state, event) {
            (State::NoteTree(ref mut state), Event::SelectNote(note_id)) => {
                state.selected = Selected::Note(note_id);
            }
            (_, Event::SelectNote(note_id)) => {
                glues.state = State::NoteTree(NoteTreeState {
                    selected: Selected::Note(note_id),
                });
            }
            (State::NoteTree(ref mut state), Event::SelectDirectory(directory_id)) => {
                state.selected = Selected::Directory(directory_id);
            }
            (_, Event::SelectDirectory(directory_id)) => {
                glues.state = State::NoteTree(NoteTreeState {
                    selected: Selected::Directory(directory_id),
                })
            }
            _ => return Err(Error::Wip("todo: NoteTree::consume".to_owned())),
        };

        Ok(())
    }

    pub fn describe(&self) -> String {
        // todo: message based on internal state
        "Notes opened".to_owned()
    }
}
