use crate::data::{Directory, Note};

pub enum Transition<'a> {
    Initialize,

    SelectNote,
    SelectDirectory {
        notes: &'a [Note],
        directories: &'a [Directory],
    },
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
