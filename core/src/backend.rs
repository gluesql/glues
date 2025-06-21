use crate::{
    Result,
    data::{Directory, Note},
    types::{DirectoryId, NoteId},
};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait CoreBackend {
    fn root_id(&self) -> DirectoryId;

    async fn fetch_directory(&mut self, directory_id: DirectoryId) -> Result<Directory>;
    async fn fetch_directories(&mut self, parent_id: DirectoryId) -> Result<Vec<Directory>>;
    async fn add_directory(&mut self, parent_id: DirectoryId, name: String) -> Result<Directory>;
    async fn remove_directory(&mut self, directory_id: DirectoryId) -> Result<()>;
    async fn move_directory(
        &mut self,
        directory_id: DirectoryId,
        parent_id: DirectoryId,
    ) -> Result<()>;
    async fn rename_directory(&mut self, directory_id: DirectoryId, name: String) -> Result<()>;

    async fn fetch_notes(&mut self, directory_id: DirectoryId) -> Result<Vec<Note>>;
    async fn fetch_note_content(&mut self, note_id: NoteId) -> Result<String>;
    async fn add_note(&mut self, directory_id: DirectoryId, name: String) -> Result<Note>;
    async fn remove_note(&mut self, note_id: NoteId) -> Result<()>;
    async fn rename_note(&mut self, note_id: NoteId, name: String) -> Result<()>;
    async fn update_note_content(&mut self, note_id: NoteId, content: String) -> Result<()>;
    async fn move_note(&mut self, note_id: NoteId, directory_id: DirectoryId) -> Result<()>;

    async fn log(&mut self, category: String, message: String) -> Result<()>;
}

pub mod local;
pub mod proxy;
