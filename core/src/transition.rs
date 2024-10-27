use {
    crate::{
        data::{Directory, Note},
        state::notebook::DirectoryItem,
        types::DirectoryId,
        Event,
    },
    strum_macros::Display,
};

#[derive(Display)]
pub enum Transition {
    #[strum(to_string = "Entry::{0}")]
    Entry(EntryTransition),

    #[strum(to_string = "Notebook::{0}")]
    Notebook(NotebookTransition),

    Log(String),
    Error(String),
}

#[derive(Display)]
pub enum EntryTransition {
    OpenNotebook,

    #[strum(to_string = "Inedible::{0}")]
    Inedible(Event),

    None,
}

#[derive(Display)]
pub enum NotebookTransition {
    OpenDirectory {
        id: DirectoryId,
        notes: Vec<Note>,
        directories: Vec<DirectoryItem>,
    },
    CloseDirectory(DirectoryId),

    RenameNote(Note),
    RenameDirectory(Directory),

    RemoveNote {
        note: Note,
        selected_directory: Directory,
    },
    RemoveDirectory {
        directory: Directory,
        selected_directory: Directory,
    },

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

    Alert(String),

    #[strum(to_string = "Inedible::{0}")]
    Inedible(Event),
    None,

    // Additional frontend action required
    SelectNext(usize),
    SelectPrev(usize),
    EditingNormalMode(NormalModeTransition),
}

#[derive(Display)]
pub enum NormalModeTransition {
    IdleMode,
    NumberingMode,
    MoveCursorDown(usize),
    MoveCursorUp(usize),
    MoveCursorBack(usize),
    MoveCursorForward(usize),
    MoveCursorWordForward(usize),
    MoveCursorWordEnd(usize),
    MoveCursorWordBack(usize),
    InsertNewLineBelow,
    InsertNewLineAbove,
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
