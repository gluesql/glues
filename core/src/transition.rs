use crate::{
    data::{Directory, Note},
    state::note_tree::DirectoryItem,
    types::DirectoryId,
};

pub enum Transition {
    None,
    Initialize,

    OpenDirectory {
        id: DirectoryId,
        notes: Vec<Note>,
        directories: Vec<DirectoryItem>,
    },
    CloseDirectory(DirectoryId),

    RenameNote(Note),
    RenameDirectory(Directory),

    RemoveNote(Note),
    RemoveDirectory(Directory),

    AddNote(Note),
    AddDirectory(Directory),

    ShowNoteActionsDialog(Note),
    ShowDirectoryActionsDialog(Directory),
}
