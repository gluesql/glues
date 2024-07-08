use crate::{data::Note, types::DirectoryId};

pub enum Event {
    Initialize,

    OpenDirectory(DirectoryId),
    CloseDirectory(DirectoryId),

    Key(KeyEvent),

    SelectNote(Note),
    SelectDirectory { id: DirectoryId, name: String },

    RenameNote(String),
    RenameDirectory(String),

    RemoveNote,

    ShowNoteActionsDialog,
    CloseNoteActionsDialog,

    ShowDirectoryActionsDialog,
    CloseDirectoryActionsDialog,
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

pub enum KeyEvent {
    Q,
    M,
}

impl From<KeyEvent> for Event {
    fn from(event: KeyEvent) -> Self {
        Self::Key(event)
    }
}
