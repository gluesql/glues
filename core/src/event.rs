use crate::{
    data::{Directory, Note},
    state::note_tree::DirectoryItem,
};

pub enum Event {
    Initialize,

    SelectNote(Note),
    SelectDirectory(DirectoryItem),
    OpenDirectory(Directory),
    CloseDirectory(Directory),
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
