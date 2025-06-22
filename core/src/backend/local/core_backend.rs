use crate::{
    Result,
    backend::CoreBackend,
    data::{Directory, Note},
    types::{DirectoryId, NoteId},
};
use async_trait::async_trait;

use super::Db;

#[async_trait(?Send)]
impl CoreBackend for Db {
    fn root_id(&self) -> DirectoryId {
        self.root_id.clone()
    }

    async fn fetch_directory(&mut self, directory_id: DirectoryId) -> Result<Directory> {
        Db::fetch_directory(self, directory_id).await
    }

    async fn fetch_directories(&mut self, parent_id: DirectoryId) -> Result<Vec<Directory>> {
        Db::fetch_directories(self, parent_id).await
    }

    async fn add_directory(&mut self, parent_id: DirectoryId, name: String) -> Result<Directory> {
        Db::add_directory(self, parent_id, name).await
    }

    async fn remove_directory(&mut self, directory_id: DirectoryId) -> Result<()> {
        Db::remove_directory(self, directory_id).await
    }

    async fn move_directory(
        &mut self,
        directory_id: DirectoryId,
        parent_id: DirectoryId,
    ) -> Result<()> {
        Db::move_directory(self, directory_id, parent_id).await
    }

    async fn reorder_directory(&mut self, directory_id: DirectoryId, order: i64) -> Result<()> {
        Db::reorder_directory(self, directory_id, order).await
    }

    async fn rename_directory(&mut self, directory_id: DirectoryId, name: String) -> Result<()> {
        Db::rename_directory(self, directory_id, name).await
    }

    async fn fetch_notes(&mut self, directory_id: DirectoryId) -> Result<Vec<Note>> {
        Db::fetch_notes(self, directory_id).await
    }

    async fn fetch_note_content(&mut self, note_id: NoteId) -> Result<String> {
        Db::fetch_note_content(self, note_id).await
    }

    async fn add_note(&mut self, directory_id: DirectoryId, name: String) -> Result<Note> {
        Db::add_note(self, directory_id, name).await
    }

    async fn remove_note(&mut self, note_id: NoteId) -> Result<()> {
        Db::remove_note(self, note_id).await
    }

    async fn rename_note(&mut self, note_id: NoteId, name: String) -> Result<()> {
        Db::rename_note(self, note_id, name).await
    }

    async fn update_note_content(&mut self, note_id: NoteId, content: String) -> Result<()> {
        Db::update_note_content(self, note_id, content).await
    }

    async fn move_note(&mut self, note_id: NoteId, directory_id: DirectoryId) -> Result<()> {
        Db::move_note(self, note_id, directory_id).await
    }

    async fn reorder_note(&mut self, note_id: NoteId, order: i64) -> Result<()> {
        Db::reorder_note(self, note_id, order).await
    }

    async fn log(&mut self, category: String, message: String) -> Result<()> {
        Db::log(self, category, message).await
    }
}
