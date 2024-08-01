use crate::{
    data::{Directory, Note},
    state::notebook::DirectoryItem,
    types::DirectoryId,
    Event,
};

pub enum Transition {
    Entry(EntryTransition),
    Notebook(NotebookTransition),
}

pub enum EntryTransition {
    Initialize,
    Inedible(Event),
}

pub enum NotebookTransition {
    OpenDirectory {
        id: DirectoryId,
        notes: Vec<Note>,
        directories: Vec<DirectoryItem>,
    },
    CloseDirectory {
        directory_id: DirectoryId,
        by_note: bool,
    },

    RenameNote(Note),
    RenameDirectory(Directory),

    RemoveNote(Note),
    RemoveDirectory(Directory),

    AddNote(Note),
    AddDirectory(Directory),

    ShowNoteActionsDialog(Note),
    ShowDirectoryActionsDialog(Directory),

    OpenNote {
        note: Note,
        content: String,
    },
    EditMode,
    ViewMode(Note),
    SelectNote(Note),
    SelectDirectory(Directory),
    UpdateNoteContent,

    Inedible(Event),
    None,
}

impl From<EntryTransition> for Transition {
    fn from(t: EntryTransition) -> Self {
        Self::Entry(t)
    }
}

impl From<NotebookTransition> for Transition {
    fn from(t: NotebookTransition) -> Self {
        Self::Notebook(t)
    }
}
