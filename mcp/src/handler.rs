use async_trait::async_trait;
use glues_core::{db::Db, proxy::ProxyServer};
use rust_mcp_sdk::{
    McpServer,
    mcp_server::ServerHandler,
    schema::{
        CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
        schema_utils::CallToolError,
    },
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::tools::NoteTools;

pub struct GluesHandler {
    pub server: Arc<Mutex<ProxyServer<Db>>>,
}

#[async_trait]
impl ServerHandler for GluesHandler {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: NoteTools::tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let tool = NoteTools::try_from(request.params).map_err(CallToolError::new)?;
        let server = self.server.clone();
        match tool {
            NoteTools::RootId(tool) => tool.call_tool(server).await,
            NoteTools::ListNotes(tool) => tool.call_tool(server).await,
            NoteTools::GetNote(tool) => tool.call_tool(server).await,
            NoteTools::AddNote(tool) => tool.call_tool(server).await,
        }
    }
}
