use crate::{data::Note, state::note_tree::DirectoryItem};

pub enum Transition<'a> {
    Initialize,

    OpenDirectory {
        notes: &'a [Note],
        directories: &'a [DirectoryItem],
    },
    CloseDirectory,
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
