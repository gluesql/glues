use crate::data::Directory;

pub enum Event {
    Initialize,

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
