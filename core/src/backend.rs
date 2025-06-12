use crate::{
    db::{CoreBackend, Db},
    proxy::ProxyClient,
    data::{Directory, Note},
    types::{DirectoryId, NoteId},
    Result,
};
use async_trait::async_trait;

/// Unified backend that can be either a local [`Db`] or a remote
/// [`ProxyClient`].
pub enum Backend {
    /// Local database backend
    Db(Db),
    /// Remote proxy backend
    Remote(ProxyClient),
}

impl From<Db> for Backend {
    fn from(db: Db) -> Self {
        Self::Db(db)
    }
}

impl From<ProxyClient> for Backend {
    fn from(client: ProxyClient) -> Self {
        Self::Remote(client)
    }
}

#[async_trait(?Send)]
impl CoreBackend for Backend {
    fn root_id(&self) -> DirectoryId {
        match self {
            Backend::Db(db) => db.root_id(),
            Backend::Remote(client) => client.root_id(),
        }
    }

    async fn fetch_directory(&mut self, directory_id: DirectoryId) -> Result<Directory> {
        match self {
            Backend::Db(db) => db.fetch_directory(directory_id).await,
            Backend::Remote(client) => client.fetch_directory(directory_id).await,
        }
    }

    async fn fetch_directories(&mut self, parent_id: DirectoryId) -> Result<Vec<Directory>> {
        match self {
            Backend::Db(db) => db.fetch_directories(parent_id).await,
            Backend::Remote(client) => client.fetch_directories(parent_id).await,
        }
    }

    async fn add_directory(&mut self, parent_id: DirectoryId, name: String) -> Result<Directory> {
        match self {
            Backend::Db(db) => db.add_directory(parent_id, name).await,
            Backend::Remote(client) => client.add_directory(parent_id, name).await,
        }
    }

    async fn remove_directory(&mut self, directory_id: DirectoryId) -> Result<()> {
        match self {
            Backend::Db(db) => db.remove_directory(directory_id).await,
            Backend::Remote(client) => client.remove_directory(directory_id).await,
        }
    }

    async fn move_directory(&mut self, directory_id: DirectoryId, parent_id: DirectoryId) -> Result<()> {
        match self {
            Backend::Db(db) => db.move_directory(directory_id, parent_id).await,
            Backend::Remote(client) => client.move_directory(directory_id, parent_id).await,
        }
    }

    async fn rename_directory(&mut self, directory_id: DirectoryId, name: String) -> Result<()> {
        match self {
            Backend::Db(db) => db.rename_directory(directory_id, name).await,
            Backend::Remote(client) => client.rename_directory(directory_id, name).await,
        }
    }

    async fn fetch_notes(&mut self, directory_id: DirectoryId) -> Result<Vec<Note>> {
        match self {
            Backend::Db(db) => db.fetch_notes(directory_id).await,
            Backend::Remote(client) => client.fetch_notes(directory_id).await,
        }
    }

    async fn fetch_note_content(&mut self, note_id: NoteId) -> Result<String> {
        match self {
            Backend::Db(db) => db.fetch_note_content(note_id).await,
            Backend::Remote(client) => client.fetch_note_content(note_id).await,
        }
    }

    async fn add_note(&mut self, directory_id: DirectoryId, name: String) -> Result<Note> {
        match self {
            Backend::Db(db) => db.add_note(directory_id, name).await,
            Backend::Remote(client) => client.add_note(directory_id, name).await,
        }
    }

    async fn remove_note(&mut self, note_id: NoteId) -> Result<()> {
        match self {
            Backend::Db(db) => db.remove_note(note_id).await,
            Backend::Remote(client) => client.remove_note(note_id).await,
        }
    }

    async fn rename_note(&mut self, note_id: NoteId, name: String) -> Result<()> {
        match self {
            Backend::Db(db) => db.rename_note(note_id, name).await,
            Backend::Remote(client) => client.rename_note(note_id, name).await,
        }
    }

    async fn update_note_content(&mut self, note_id: NoteId, content: String) -> Result<()> {
        match self {
            Backend::Db(db) => db.update_note_content(note_id, content).await,
            Backend::Remote(client) => client.update_note_content(note_id, content).await,
        }
    }

    async fn move_note(&mut self, note_id: NoteId, directory_id: DirectoryId) -> Result<()> {
        match self {
            Backend::Db(db) => db.move_note(note_id, directory_id).await,
            Backend::Remote(client) => client.move_note(note_id, directory_id).await,
        }
    }

    async fn log(&mut self, category: String, message: String) -> Result<()> {
        match self {
            Backend::Db(db) => db.log(category, message).await,
            Backend::Remote(client) => client.log(category, message).await,
        }
    }
}
