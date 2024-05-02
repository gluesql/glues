use crate::types::{DirectoryId, NoteId};

#[derive(Clone, Debug)]
pub struct Note {
    pub id: NoteId,
    pub directory_id: DirectoryId,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Directory {
    pub id: DirectoryId,
    pub parent_id: DirectoryId,
    pub name: String,
}
