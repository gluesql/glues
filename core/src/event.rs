use {
    crate::{
        data::{Directory, Note},
        types::DirectoryId,
    },
    strum_macros::Display,
};

#[derive(Clone, Debug, Display)]
pub enum Event {
    #[strum(to_string = "Key::{0}")]
    Key(KeyEvent),

    #[strum(to_string = "Entry::{0}")]
    Entry(EntryEvent),

    #[strum(to_string = "Notebook::{0}")]
    Notebook(NotebookEvent),

    Cancel,
}

#[derive(Clone, Debug, Display)]
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
    OpenMongo {
        conn_str: String,
        db_name: String,
    },
}

#[derive(Clone, Debug, Display)]
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
    Num(NumKey),
    Left,
    Right,
    Esc,
}

#[derive(Clone, Copy, Debug, Display)]
pub enum NumKey {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
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

impl From<NumKey> for KeyEvent {
    fn from(num_key: NumKey) -> Self {
        KeyEvent::Num(num_key)
    }
}

impl From<NumKey> for usize {
    fn from(num_key: NumKey) -> Self {
        match num_key {
            NumKey::One => 1,
            NumKey::Two => 2,
            NumKey::Three => 3,
            NumKey::Four => 4,
            NumKey::Five => 5,
            NumKey::Six => 6,
            NumKey::Seven => 7,
            NumKey::Eight => 8,
            NumKey::Nine => 9,
            NumKey::Zero => 0,
        }
    }
}

impl std::ops::Add<usize> for NumKey {
    type Output = usize;

    fn add(self, rhs: usize) -> Self::Output {
        let n = usize::from(self).saturating_add(rhs);

        if n > u16::MAX as usize {
            u16::MAX as usize
        } else {
            n
        }
    }
}
