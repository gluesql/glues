mod handler;
mod tools;

use std::sync::{Arc, mpsc::channel};
use std::time::Duration;

use clap::Parser;
use tokio::sync::Mutex;

use glues_core::{db::Db, proxy::ProxyServer};
use rust_mcp_sdk::{
    error::SdkResult,
    mcp_server::{HyperServerOptions, hyper_server},
    schema::{
        Implementation, InitializeResult, LATEST_PROTOCOL_VERSION, ServerCapabilities,
        ServerCapabilitiesTools,
    },
};

use handler::GluesHandler;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() -> SdkResult<()> {
    color_eyre::install().ok();
    let cli = Cli::parse();

    let (tx, _rx) = channel();
    let db = Db::memory(tx)
        .await
        .map_err(|e| rust_mcp_sdk::error::McpSdkError::AnyError(Box::new(e)))?;
    let server = Arc::new(Mutex::new(ProxyServer::new(db)));

    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Glues MCP".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some("use tools to manage notes".to_string()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    let handler = GluesHandler {
        server: server.clone(),
    };

    let mcp_server = hyper_server::create_server(
        server_details,
        handler,
        HyperServerOptions {
            host: cli.host,
            port: cli.port,
            ping_interval: Duration::from_secs(5),
            ..Default::default()
        },
    );

    mcp_server.start().await?;
    Ok(())
}
