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
    AddDirectory(String),

    OpenNote,
    EditNote,

    UpdateNoteContent(String),
}

#[derive(Clone, Copy)]
pub enum KeyEvent {
    B,
    E,
    H,
    J,
    K,
    L,
    M,
    O,
    Left,
    Right,
    Esc,
}

impl From<KeyEvent> for Event {
    fn from(event: KeyEvent) -> Self {
        Self::Key(event)
    }
}
