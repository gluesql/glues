use crate::{
    data::{Directory, Note},
    state::note_tree::DirectoryItem,
    types::DirectoryId,
    Error, Result,
};

pub enum Transition {
    None,
    Initialize,

    OpenDirectory {
        id: DirectoryId,
        notes: Vec<Note>,
        directories: Vec<DirectoryItem>,
    },
    CloseDirectory(DirectoryId),

    RenameNote(Note),
    RenameDirectory(Directory),

    RemoveNote(Note),
    RemoveDirectory(Directory),

    AddNote(Note),
    AddDirectory(Directory),

    ShowNoteActionsDialog(ShowNoteActionsDialog),
    ShowDirectoryActionsDialog(ShowDirectoryActionsDialog),
}

pub struct ShowNoteActionsDialog {
    pub note: Note,
}

pub struct ShowDirectoryActionsDialog {
    pub directory: Directory,
}

pub trait GetTransition<T> {
    fn get_transition(self) -> Result<T>;
}

impl GetTransition<()> for Transition {
    fn get_transition(self) -> Result<()> {
        Ok(())
    }
}

impl GetTransition<ShowNoteActionsDialog> for Transition {
    fn get_transition(self) -> Result<ShowNoteActionsDialog> {
        match self {
            Self::ShowNoteActionsDialog(v) => Ok(v),
            _ => Err(Error::Wip(
                "Transition::get_transition for transition::ShowNoteActionsDialog failed"
                    .to_owned(),
            )),
        }
    }
}

impl From<ShowNoteActionsDialog> for Transition {
    fn from(v: ShowNoteActionsDialog) -> Self {
        Self::ShowNoteActionsDialog(v)
    }
}

impl GetTransition<ShowDirectoryActionsDialog> for Transition {
    fn get_transition(self) -> Result<ShowDirectoryActionsDialog> {
        match self {
            Self::ShowDirectoryActionsDialog(v) => Ok(v),
            _ => Err(Error::Wip(
                "Transition::get_transition for transition::ShowDirectoryActionsDialog failed"
                    .to_owned(),
            )),
        }
    }
}

impl From<ShowDirectoryActionsDialog> for Transition {
    fn from(v: ShowDirectoryActionsDialog) -> Self {
        Self::ShowDirectoryActionsDialog(v)
    }
}
