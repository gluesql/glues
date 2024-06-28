use crate::{data::Note, state::note_tree::DirectoryItem};

pub enum Event {
    Initialize,

    SelectNote(Note),
    SelectDirectory(DirectoryItem),
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
