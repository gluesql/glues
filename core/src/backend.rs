use crate::{
    Error, Result,
    data::{Directory, Note},
    types::{DirectoryId, NoteId},
};
use async_trait::async_trait;
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use {
    gluesql::gluesql_git_storage::{GitStorage, StorageType},
    reqwest::blocking::Client,
    std::time::Duration,
};

#[derive(Clone)]
pub enum SyncJob {
    #[cfg(not(target_arch = "wasm32"))]
    Git {
        path: PathBuf,
        remote: String,
        branch: String,
    },
    #[cfg(not(target_arch = "wasm32"))]
    Proxy {
        url: String,
        auth_token: Option<String>,
    },
}

impl SyncJob {
    pub fn run(self) -> Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        match self {
            SyncJob::Git {
                path,
                remote,
                branch,
            } => {
                let mut storage = GitStorage::open(path, StorageType::File)?;
                storage.set_remote(remote);
                storage.set_branch(branch);
                storage.pull()?;
                storage.push()?;
            }
            SyncJob::Proxy { url, auth_token } => {
                use crate::backend::proxy::{
                    request::ProxyRequest,
                    response::{ProxyResponse, ResultPayload},
                };

                let client = Client::builder().timeout(Duration::from_secs(30)).build()?;
                let mut request = client.post(&url).json(&ProxyRequest::Sync);
                if let Some(token) = auth_token.as_ref() {
                    request = request.bearer_auth(token);
                }

                let response = request.send()?;
                if !response.status().is_success() {
                    return Err(Error::Proxy(format!(
                        "sync failed with status {}",
                        response.status()
                    )));
                }

                match response.json()? {
                    ProxyResponse::Ok(ResultPayload::Unit) => {}
                    ProxyResponse::Ok(_) => {
                        return Err(Error::InvalidResponse(
                            "invalid sync response payload".to_owned(),
                        ));
                    }
                    ProxyResponse::Err(message) => {
                        return Err(Error::Proxy(message));
                    }
                }
            }
        }

        Ok(())
    }
}

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

    fn sync_job(&self) -> Option<SyncJob>;
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

    fn sync_job(&self) -> Option<SyncJob>;
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

    fn sync_job(&self) -> Option<SyncJob> {
        (**self).sync_job()
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

    fn sync_job(&self) -> Option<SyncJob> {
        (**self).sync_job()
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

    fn sync_job(&self) -> Option<SyncJob> {
        (**self).sync_job()
    }
}

#[cfg(target_arch = "wasm32")]
pub type BackendBox = Box<dyn CoreBackend>;
#[cfg(not(target_arch = "wasm32"))]
pub type BackendBox = Box<dyn CoreBackend + Send>;

pub mod local;
pub mod proxy;
