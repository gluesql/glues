use crate::{
    data::{Directory, Note},
    state::notebook::DirectoryItem,
    types::DirectoryId,
    Event,
};

pub enum Transition {
    None,
    Initialize,
    Inedible(Event),

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
}
