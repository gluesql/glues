use crate::types::{DirectoryId, NoteId};

pub enum Event {
    Initialize,

    SelectNote(NoteId),
    SelectDirectory(DirectoryId),
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
