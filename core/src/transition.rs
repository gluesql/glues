use {
    crate::{
        Event,
        data::{Directory, Note},
        state::notebook::DirectoryItem,
        types::{DirectoryId, NoteId},
    },
    strum_macros::Display,
};

#[derive(Display)]
pub enum Transition {
    #[strum(to_string = "Keymap::{0}")]
    Keymap(KeymapTransition),

    #[strum(to_string = "Entry::{0}")]
    Entry(EntryTransition),

    #[strum(to_string = "Notebook::{0}")]
    Notebook(NotebookTransition),

    Log(String),
    Error(String),
}

#[derive(Display)]
pub enum KeymapTransition {
    Show,
    Hide,
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
    ViewMode(Note),
    BrowseNoteTree,
    FocusEditor,

    UpdateNoteContent(NoteId),

    Alert(String),

    #[strum(to_string = "Inedible::{0}")]
    Inedible(Event),
    None,

    NoteTree(NoteTreeTransition),
    EditingNormalMode(NormalModeTransition),
    EditingVisualMode(VisualModeTransition),
    ShowVimKeymap(VimKeymapKind),
}

pub enum NoteTreeTransition {
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

    MoveMode(MoveModeTransition),

    OpenNote {
        note: Note,
        content: String,
    },

    SelectNext(usize),
    SelectPrev(usize),
    SelectFirst,
    SelectLast,

    SelectNextDirectory,
    SelectPrevDirectory,

    ExpandWidth(usize),
    ShrinkWidth(usize),
    GatewayMode,
}

pub enum MoveModeTransition {
    Enter,
    SelectNext,
    SelectPrev,
    SelectLast,
    RequestCommit,
    Commit,
    Cancel,
}

#[derive(Clone, Copy, Display)]
pub enum VimKeymapKind {
    NormalIdle,
    NormalNumbering,
    NormalDelete,
    NormalDelete2,
    NormalChange,
    NormalChange2,
    VisualIdle,
    VisualNumbering,
}

#[derive(Display)]
pub enum NormalModeTransition {
    IdleMode,
    ToggleMode,
    ToggleTabCloseMode,
    NumberingMode,
    GatewayMode,
    YankMode,
    DeleteMode,
    DeleteInsideMode,
    ChangeMode,
    ChangeInsideMode,
    ScrollMode,

    // toggle mode
    NextTab(NoteId),
    PrevTab(NoteId),
    CloseTab(NoteId),
    MoveTabNext(usize),
    MoveTabPrev(usize),
    ToggleLineNumbers,
    ToggleBrowser,

    // toggle tab close mode
    CloseRightTabs(usize),
    CloseLeftTabs(usize),

    MoveCursorDown(usize),
    MoveCursorUp(usize),
    MoveCursorBack(usize),
    MoveCursorForward(usize),
    MoveCursorWordForward(usize),
    MoveCursorWordEnd(usize),
    MoveCursorWordBack(usize),
    MoveCursorLineStart,
    MoveCursorLineEnd,
    MoveCursorLineNonEmptyStart,
    MoveCursorTop,
    MoveCursorBottom,
    MoveCursorToLine(usize),
    ScrollCenter,
    ScrollTop,
    ScrollBottom,
    InsertAtCursor,
    InsertAtLineStart,
    InsertAfterCursor,
    InsertAtLineEnd,
    InsertNewLineBelow,
    InsertNewLineAbove,
    DeleteChars(usize),
    DeleteCharsBack(usize),
    DeleteLines(usize),
    DeleteLinesAndInsert(usize),
    DeleteWordEnd(usize),
    DeleteWordBack(usize),
    DeleteLineStart,
    DeleteLineEnd(usize),
    Paste,
    Undo,
    Redo,
    YankLines(usize),
    DeleteInsideWord(usize),
    SwitchCase,
}

#[derive(Display)]
pub enum VisualModeTransition {
    IdleMode,
    NumberingMode,
    GatewayMode,
    MoveCursorDown(usize),
    MoveCursorUp(usize),
    MoveCursorBack(usize),
    MoveCursorForward(usize),
    MoveCursorWordForward(usize),
    MoveCursorWordEnd(usize),
    MoveCursorWordBack(usize),
    MoveCursorLineStart,
    MoveCursorLineEnd,
    MoveCursorLineNonEmptyStart,
    MoveCursorTop,
    MoveCursorBottom,
    MoveCursorToLine(usize),
    YankSelection,
    DeleteSelection,
    DeleteSelectionAndInsertMode,
    SwitchCase,
    ToUppercase,
    ToLowercase,
}

impl From<KeymapTransition> for Transition {
    fn from(t: KeymapTransition) -> Self {
        Self::Keymap(t)
    }
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
