use crate::types::{DirectoryId, NoteId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: NoteId,
    pub directory_id: DirectoryId,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Directory {
    pub id: DirectoryId,
    pub parent_id: DirectoryId,
    pub name: String,
}
