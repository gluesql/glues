use crate::{
    Result,
    data::{Directory, Note},
    types::{DirectoryId, NoteId},
};
use async_trait::async_trait;

#[cfg(target_arch = "wasm32")]
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

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
pub trait CoreBackend: Send {
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

#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
impl CoreBackend for Box<dyn CoreBackend> {
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

#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
impl CoreBackend for Box<dyn CoreBackend + Send> {
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

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl CoreBackend for Box<dyn CoreBackend + Send> {
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

#[cfg(target_arch = "wasm32")]
pub type BackendBox = Box<dyn CoreBackend>;
#[cfg(not(target_arch = "wasm32"))]
pub type BackendBox = Box<dyn CoreBackend + Send>;

pub mod local;
pub mod proxy;
