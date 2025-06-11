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

use super::Db;

#[async_trait(?Send)]
impl<B: CoreBackend + ?Sized> CoreBackend for Box<B> {
    fn root_id(&self) -> DirectoryId {
        (**self).root_id()
    }

    async fn fetch_directory(&mut self, directory_id: DirectoryId) -> Result<Directory> {
        (**self).fetch_directory(directory_id).await
    }

    async fn fetch_directories(&mut self, parent_id: DirectoryId) -> Result<Vec<Directory>> {
        (**self).fetch_directories(parent_id).await
    }

    async fn add_directory(&mut self, parent_id: DirectoryId, name: String) -> Result<Directory> {
        (**self).add_directory(parent_id, name).await
    }

    async fn remove_directory(&mut self, directory_id: DirectoryId) -> Result<()> {
        (**self).remove_directory(directory_id).await
    }

    async fn move_directory(
        &mut self,
        directory_id: DirectoryId,
        parent_id: DirectoryId,
    ) -> Result<()> {
        (**self).move_directory(directory_id, parent_id).await
    }

    async fn rename_directory(&mut self, directory_id: DirectoryId, name: String) -> Result<()> {
        (**self).rename_directory(directory_id, name).await
    }

    async fn fetch_notes(&mut self, directory_id: DirectoryId) -> Result<Vec<Note>> {
        (**self).fetch_notes(directory_id).await
    }

    async fn fetch_note_content(&mut self, note_id: NoteId) -> Result<String> {
        (**self).fetch_note_content(note_id).await
    }

    async fn add_note(&mut self, directory_id: DirectoryId, name: String) -> Result<Note> {
        (**self).add_note(directory_id, name).await
    }

    async fn remove_note(&mut self, note_id: NoteId) -> Result<()> {
        (**self).remove_note(note_id).await
    }

    async fn rename_note(&mut self, note_id: NoteId, name: String) -> Result<()> {
        (**self).rename_note(note_id, name).await
    }

    async fn update_note_content(&mut self, note_id: NoteId, content: String) -> Result<()> {
        (**self).update_note_content(note_id, content).await
    }

    async fn move_note(&mut self, note_id: NoteId, directory_id: DirectoryId) -> Result<()> {
        (**self).move_note(note_id, directory_id).await
    }

    async fn log(&mut self, category: String, message: String) -> Result<()> {
        (**self).log(category, message).await
    }
}

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

    async fn log(&mut self, category: String, message: String) -> Result<()> {
        Db::log(self, category, message).await
    }
}
