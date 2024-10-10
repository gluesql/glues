use {
    crate::{
        data::{Directory, Note},
        types::DirectoryId,
    },
    strum_macros::Display,
};

#[derive(Debug, Display)]
pub enum Event {
    #[strum(to_string = "Key::{0}")]
    Key(KeyEvent),

    #[strum(to_string = "Entry::{0}")]
    Entry(EntryEvent),

    #[strum(to_string = "Notebook::{0}")]
    Notebook(NotebookEvent),

    Cancel,
}

#[derive(Debug, Display)]
pub enum EntryEvent {
    OpenMemory,
    OpenCsv(String),
    OpenJson(String),
    OpenFile(String),
    OpenGit {
        path: String,
        remote: String,
        branch: String,
    },
}

#[derive(Debug, Display)]
pub enum NotebookEvent {
    OpenDirectory(DirectoryId),
    CloseDirectory(DirectoryId),

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

    AddNote(String),
    AddDirectory(String),

    OpenNote,
    EditNote,
    ViewNote,
    BrowseNoteTree,

    UpdateNoteContent(String),

    CloseEntryDialog,
}

#[derive(Clone, Copy, Debug, Display)]
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

impl From<EntryEvent> for Event {
    fn from(event: EntryEvent) -> Self {
        Self::Entry(event)
    }
}

impl From<NotebookEvent> for Event {
    fn from(event: NotebookEvent) -> Self {
        Self::Notebook(event)
    }
}

impl From<KeyEvent> for Event {
    fn from(event: KeyEvent) -> Self {
        Self::Key(event)
    }
}
