use {
    clap::{Args, Parser, Subcommand},
    glues_core::types::DirectoryId,
    std::net::SocketAddr,
};

#[derive(Clone, Args)]
pub struct ServerArgs {
    #[arg(long, default_value = "127.0.0.1:4000")]
    pub listen: SocketAddr,

    #[arg(long, env = "GLUES_SERVER_TOKEN")]
    pub auth_token: Option<String>,

    #[arg(long)]
    pub allowed_directory: Option<DirectoryId>,

    #[command(subcommand)]
    pub storage: StorageCommand,
}

#[derive(Parser)]
#[command(author, version, about = "Glues proxy server")]
struct Cli {
    #[command(flatten)]
    args: ServerArgs,
}

#[derive(Subcommand, Clone)]
pub enum StorageCommand {
    /// In-memory storage (data resets on restart)
    Memory,
    /// File storage backend rooted at the given path
    File { path: String },
    /// redb single-file storage backend
    Redb { path: String },
    /// Git storage backend
    Git {
        path: String,
        remote: String,
        branch: String,
    },
    /// MongoDB storage backend
    Mongo { conn_str: String, db_name: String },
}

pub fn parse_args() -> ServerArgs {
    Cli::parse().args
}
