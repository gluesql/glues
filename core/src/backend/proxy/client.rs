use super::request::ProxyRequest;
use super::response::{ProxyResponse, ResultPayload};
use crate::{
    Error, Result,
    backend::CoreBackend,
    data::{Directory, Note},
    types::{DirectoryId, NoteId},
};
use async_trait::async_trait;
use reqwest::Client;

pub struct ProxyClient {
    url: String,
    client: Client,
    root_id: DirectoryId,
}

impl ProxyClient {
    pub async fn connect<U: Into<String>>(url: U) -> Result<Self> {
        let url = url.into();
        let client = Client::new();
        let resp = client.post(&url).json(&ProxyRequest::RootId).send().await?;
        let resp: ProxyResponse = resp.json().await?;
        let root_id = match resp {
            ProxyResponse::Ok(ResultPayload::Id(id)) => id,
            ProxyResponse::Err(e) => return Err(Error::Proxy(e)),
            _ => return Err(Error::InvalidResponse("invalid response".to_owned())),
        };

        Ok(Self {
            url,
            client,
            root_id,
        })
    }

    async fn rpc(&self, req: ProxyRequest) -> Result<ProxyResponse> {
        let resp = self.client.post(&self.url).json(&req).send().await?;
        let resp: ProxyResponse = resp.json().await?;
        Ok(resp)
    }
}

#[async_trait(?Send)]
impl CoreBackend for ProxyClient {
    fn root_id(&self) -> DirectoryId {
        self.root_id.clone()
    }

    async fn fetch_directory(&mut self, directory_id: DirectoryId) -> Result<Directory> {
        match self
            .rpc(ProxyRequest::FetchDirectory { directory_id })
            .await?
        {
            ProxyResponse::Ok(ResultPayload::Directory(dir)) => Ok(dir),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn fetch_directories(&mut self, parent_id: DirectoryId) -> Result<Vec<Directory>> {
        match self
            .rpc(ProxyRequest::FetchDirectories { parent_id })
            .await?
        {
            ProxyResponse::Ok(ResultPayload::Directories(dirs)) => Ok(dirs),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn add_directory(&mut self, parent_id: DirectoryId, name: String) -> Result<Directory> {
        match self
            .rpc(ProxyRequest::AddDirectory { parent_id, name })
            .await?
        {
            ProxyResponse::Ok(ResultPayload::Directory(dir)) => Ok(dir),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn remove_directory(&mut self, directory_id: DirectoryId) -> Result<()> {
        match self
            .rpc(ProxyRequest::RemoveDirectory { directory_id })
            .await?
        {
            ProxyResponse::Ok(ResultPayload::Unit) => Ok(()),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn move_directory(
        &mut self,
        directory_id: DirectoryId,
        parent_id: DirectoryId,
    ) -> Result<()> {
        match self
            .rpc(ProxyRequest::MoveDirectory {
                directory_id,
                parent_id,
            })
            .await?
        {
            ProxyResponse::Ok(ResultPayload::Unit) => Ok(()),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn rename_directory(&mut self, directory_id: DirectoryId, name: String) -> Result<()> {
        match self
            .rpc(ProxyRequest::RenameDirectory { directory_id, name })
            .await?
        {
            ProxyResponse::Ok(ResultPayload::Unit) => Ok(()),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn fetch_notes(&mut self, directory_id: DirectoryId) -> Result<Vec<Note>> {
        match self.rpc(ProxyRequest::FetchNotes { directory_id }).await? {
            ProxyResponse::Ok(ResultPayload::Notes(notes)) => Ok(notes),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn fetch_note_content(&mut self, note_id: NoteId) -> Result<String> {
        match self.rpc(ProxyRequest::FetchNoteContent { note_id }).await? {
            ProxyResponse::Ok(ResultPayload::Text(text)) => Ok(text),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn add_note(&mut self, directory_id: DirectoryId, name: String) -> Result<Note> {
        match self
            .rpc(ProxyRequest::AddNote { directory_id, name })
            .await?
        {
            ProxyResponse::Ok(ResultPayload::Note(note)) => Ok(note),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn remove_note(&mut self, note_id: NoteId) -> Result<()> {
        match self.rpc(ProxyRequest::RemoveNote { note_id }).await? {
            ProxyResponse::Ok(ResultPayload::Unit) => Ok(()),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn rename_note(&mut self, note_id: NoteId, name: String) -> Result<()> {
        match self.rpc(ProxyRequest::RenameNote { note_id, name }).await? {
            ProxyResponse::Ok(ResultPayload::Unit) => Ok(()),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn update_note_content(&mut self, note_id: NoteId, content: String) -> Result<()> {
        match self
            .rpc(ProxyRequest::UpdateNoteContent { note_id, content })
            .await?
        {
            ProxyResponse::Ok(ResultPayload::Unit) => Ok(()),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn move_note(&mut self, note_id: NoteId, directory_id: DirectoryId) -> Result<()> {
        match self
            .rpc(ProxyRequest::MoveNote {
                note_id,
                directory_id,
            })
            .await?
        {
            ProxyResponse::Ok(ResultPayload::Unit) => Ok(()),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }

    async fn log(&mut self, category: String, message: String) -> Result<()> {
        match self.rpc(ProxyRequest::Log { category, message }).await? {
            ProxyResponse::Ok(ResultPayload::Unit) => Ok(()),
            ProxyResponse::Err(e) => Err(Error::Proxy(e)),
            _ => Err(Error::InvalidResponse("invalid response".to_owned())),
        }
    }
}
