use crate::{
    data::{Directory, Note},
    state::note_tree::DirectoryItem,
    types::NoteId,
    Error, Result,
};

pub enum Transition<'a> {
    None,

    OpenDirectory(OpenDirectory<'a>),
    CloseDirectory,

    RenameNote { id: NoteId, name: String },

    ShowNoteActionsDialog(ShowNoteActionsDialog),
    ShowDirectoryActionsDialog(ShowDirectoryActionsDialog),
    /*
    AddNote {
        directory_id: DirectoryId,
        name: String,
    },
    AddDirectory,

    RemoveNote,
    RemoveDirectory,

    RenameNote,
    RenameDirectory,
    */
}

pub struct OpenDirectory<'a> {
    pub notes: &'a [Note],
    pub directories: &'a [DirectoryItem],
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

impl<'a> GetTransition<()> for Transition<'a> {
    fn get_transition(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> GetTransition<OpenDirectory<'a>> for Transition<'a> {
    fn get_transition(self) -> Result<OpenDirectory<'a>> {
        match self {
            Self::OpenDirectory(v) => Ok(v),
            _ => Err(Error::Wip(
                "Transition::get_transition for transition::OpenDirectory failed".to_owned(),
            )),
        }
    }
}

impl<'a> From<OpenDirectory<'a>> for Transition<'a> {
    fn from(v: OpenDirectory<'a>) -> Self {
        Self::OpenDirectory(v)
    }
}

impl<'a> GetTransition<ShowNoteActionsDialog> for Transition<'a> {
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

impl<'a> From<ShowNoteActionsDialog> for Transition<'a> {
    fn from(v: ShowNoteActionsDialog) -> Self {
        Self::ShowNoteActionsDialog(v)
    }
}

impl<'a> GetTransition<ShowDirectoryActionsDialog> for Transition<'a> {
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

impl<'a> From<ShowDirectoryActionsDialog> for Transition<'a> {
    fn from(v: ShowDirectoryActionsDialog) -> Self {
        Self::ShowDirectoryActionsDialog(v)
    }
}
