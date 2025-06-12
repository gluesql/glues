use super::request::ProxyRequest;
use super::response::{ProxyResponse, ResultPayload};
use crate::{backend::Backend, db::CoreBackend};

pub struct ProxyServer {
    pub db: Backend,
}

impl ProxyServer {
    pub fn new(db: Backend) -> Self {
        Self { db }
    }

    pub async fn handle(&mut self, req: ProxyRequest) -> ProxyResponse {
        use ProxyRequest::*;
        match req {
            RootId => ProxyResponse::Ok(ResultPayload::Id(self.db.root_id())),
            FetchDirectory { directory_id } => match self.db.fetch_directory(directory_id).await {
                Ok(dir) => ProxyResponse::Ok(ResultPayload::Directory(dir)),
                Err(e) => ProxyResponse::Err(e.to_string()),
            },
            FetchDirectories { parent_id } => match self.db.fetch_directories(parent_id).await {
                Ok(dirs) => ProxyResponse::Ok(ResultPayload::Directories(dirs)),
                Err(e) => ProxyResponse::Err(e.to_string()),
            },
            AddDirectory { parent_id, name } => {
                match self.db.add_directory(parent_id, name).await {
                    Ok(dir) => ProxyResponse::Ok(ResultPayload::Directory(dir)),
                    Err(e) => ProxyResponse::Err(e.to_string()),
                }
            }
            RemoveDirectory { directory_id } => {
                match self.db.remove_directory(directory_id).await {
                    Ok(()) => ProxyResponse::Ok(ResultPayload::Unit),
                    Err(e) => ProxyResponse::Err(e.to_string()),
                }
            }
            MoveDirectory {
                directory_id,
                parent_id,
            } => match self.db.move_directory(directory_id, parent_id).await {
                Ok(()) => ProxyResponse::Ok(ResultPayload::Unit),
                Err(e) => ProxyResponse::Err(e.to_string()),
            },
            RenameDirectory { directory_id, name } => {
                match self.db.rename_directory(directory_id, name).await {
                    Ok(()) => ProxyResponse::Ok(ResultPayload::Unit),
                    Err(e) => ProxyResponse::Err(e.to_string()),
                }
            }
            FetchNotes { directory_id } => match self.db.fetch_notes(directory_id).await {
                Ok(notes) => ProxyResponse::Ok(ResultPayload::Notes(notes)),
                Err(e) => ProxyResponse::Err(e.to_string()),
            },
            FetchNoteContent { note_id } => match self.db.fetch_note_content(note_id).await {
                Ok(content) => ProxyResponse::Ok(ResultPayload::Text(content)),
                Err(e) => ProxyResponse::Err(e.to_string()),
            },
            AddNote { directory_id, name } => match self.db.add_note(directory_id, name).await {
                Ok(note) => ProxyResponse::Ok(ResultPayload::Note(note)),
                Err(e) => ProxyResponse::Err(e.to_string()),
            },
            RemoveNote { note_id } => match self.db.remove_note(note_id).await {
                Ok(()) => ProxyResponse::Ok(ResultPayload::Unit),
                Err(e) => ProxyResponse::Err(e.to_string()),
            },
            RenameNote { note_id, name } => match self.db.rename_note(note_id, name).await {
                Ok(()) => ProxyResponse::Ok(ResultPayload::Unit),
                Err(e) => ProxyResponse::Err(e.to_string()),
            },
            UpdateNoteContent { note_id, content } => {
                match self.db.update_note_content(note_id, content).await {
                    Ok(()) => ProxyResponse::Ok(ResultPayload::Unit),
                    Err(e) => ProxyResponse::Err(e.to_string()),
                }
            }
            MoveNote {
                note_id,
                directory_id,
            } => match self.db.move_note(note_id, directory_id).await {
                Ok(()) => ProxyResponse::Ok(ResultPayload::Unit),
                Err(e) => ProxyResponse::Err(e.to_string()),
            },
            Log { category, message } => match self.db.log(category, message).await {
                Ok(()) => ProxyResponse::Ok(ResultPayload::Unit),
                Err(e) => ProxyResponse::Err(e.to_string()),
            },
        }
    }
}
