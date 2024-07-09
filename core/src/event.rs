use crate::{
    data::{Directory, Note},
    types::DirectoryId,
};

pub enum Event {
    Initialize,

    OpenDirectory(DirectoryId),
    CloseDirectory(DirectoryId),

    Key(KeyEvent),

    SelectNote(Note),
    SelectDirectory(Directory),

    RenameNote(String),
    RenameDirectory(String),

    RemoveNote,
    RemoveDirectory,

    ShowNoteActionsDialog,
    CloseNoteActionsDialog,

    ShowDirectoryActionsDialog,
    CloseDirectoryActionsDialog,

    Cancel,

    AddNote(String),
    /*
    AddDirectory,
    */
}

pub enum KeyEvent {
    Q,
    M,
}

impl From<KeyEvent> for Event {
    fn from(event: KeyEvent) -> Self {
        Self::Key(event)
    }
}
