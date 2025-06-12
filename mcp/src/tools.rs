use glues_core::{db::Db, proxy::ProxyServer};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[mcp_tool(name = "list_notes", description = "List notes in a directory")]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListNotes {
    pub directory_id: String,
}

impl ListNotes {
    pub async fn call_tool(
        &self,
        server: Arc<Mutex<ProxyServer<Db>>>,
    ) -> Result<CallToolResult, CallToolError> {
        let dir = self.directory_id.clone();
        let result = tokio::task::spawn_blocking(move || {
            let handle = tokio::runtime::Handle::current();
            handle.block_on(async {
                let mut srv = server.blocking_lock();
                srv.db.fetch_notes(dir).await
            })
        })
        .await
        .unwrap();
        match result {
            Ok(notes) => Ok(CallToolResult::text_content(
                serde_json::to_string(&notes).unwrap(),
                None,
            )),
            Err(e) => Err(CallToolError::new(e)),
        }
    }
}

#[mcp_tool(name = "get_note", description = "Fetch note content")]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GetNote {
    pub note_id: String,
}

impl GetNote {
    pub async fn call_tool(
        &self,
        server: Arc<Mutex<ProxyServer<Db>>>,
    ) -> Result<CallToolResult, CallToolError> {
        let note_id = self.note_id.clone();
        let result = tokio::task::spawn_blocking(move || {
            let handle = tokio::runtime::Handle::current();
            handle.block_on(async {
                let mut srv = server.blocking_lock();
                srv.db.fetch_note_content(note_id).await
            })
        })
        .await
        .unwrap();
        match result {
            Ok(content) => Ok(CallToolResult::text_content(content, None)),
            Err(e) => Err(CallToolError::new(e)),
        }
    }
}

#[mcp_tool(name = "add_note", description = "Create a new note")]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AddNote {
    pub directory_id: String,
    pub name: String,
}

impl AddNote {
    pub async fn call_tool(
        &self,
        server: Arc<Mutex<ProxyServer<Db>>>,
    ) -> Result<CallToolResult, CallToolError> {
        let dir = self.directory_id.clone();
        let name = self.name.clone();
        let result = tokio::task::spawn_blocking(move || {
            let handle = tokio::runtime::Handle::current();
            handle.block_on(async {
                let mut srv = server.blocking_lock();
                srv.db.add_note(dir, name).await
            })
        })
        .await
        .unwrap();
        match result {
            Ok(note) => Ok(CallToolResult::text_content(
                serde_json::to_string(&note).unwrap(),
                None,
            )),
            Err(e) => Err(CallToolError::new(e)),
        }
    }
}

use rust_mcp_sdk::tool_box;

tool_box!(NoteTools, [ListNotes, GetNote, AddNote]);
