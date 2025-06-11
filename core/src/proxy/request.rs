use crate::types::{DirectoryId, NoteId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method", content = "data")]
pub enum ProxyRequest {
    RootId,
    FetchDirectory { directory_id: DirectoryId },
    FetchDirectories { parent_id: DirectoryId },
    AddDirectory { parent_id: DirectoryId, name: String },
    RemoveDirectory { directory_id: DirectoryId },
    MoveDirectory { directory_id: DirectoryId, parent_id: DirectoryId },
    RenameDirectory { directory_id: DirectoryId, name: String },
    FetchNotes { directory_id: DirectoryId },
    FetchNoteContent { note_id: NoteId },
    AddNote { directory_id: DirectoryId, name: String },
    RemoveNote { note_id: NoteId },
    RenameNote { note_id: NoteId, name: String },
    UpdateNoteContent { note_id: NoteId, content: String },
    MoveNote { note_id: NoteId, directory_id: DirectoryId },
    Log { category: String, message: String },
}
