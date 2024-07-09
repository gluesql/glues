use crate::{
    data::{Directory, Note},
    state::note_tree::DirectoryItem,
    types::DirectoryId,
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

impl From<ShowNoteActionsDialog> for Transition {
    fn from(v: ShowNoteActionsDialog) -> Self {
        Self::ShowNoteActionsDialog(v)
    }
}

impl From<ShowDirectoryActionsDialog> for Transition {
    fn from(v: ShowDirectoryActionsDialog) -> Self {
        Self::ShowDirectoryActionsDialog(v)
    }
}
