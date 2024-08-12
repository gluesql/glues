use crate::{
    data::{Directory, Note},
    types::DirectoryId,
};

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
    Entry(EntryEvent),
    Notebook(NotebookEvent),

    Cancel,
}

#[derive(Debug)]
pub enum EntryEvent {
    OpenMemory,
    OpenCsv(String),
    OpenJson(String),
    OpenFile(String),
}

#[derive(Debug)]
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

    UpdateNoteContent(String),

    CloseEntryDialog,
}

#[derive(Clone, Copy, Debug)]
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
