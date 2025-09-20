use {
    clap::{Parser, Subcommand},
    color_eyre::Result,
    glues_server::ServerArgs,
    glues_tui::cli,
};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Unified entry point for Glues",
    long_about = None,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run the Glues proxy server
    Server(ServerArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let Cli { command } = Cli::parse();

    match command {
        Some(Command::Server(args)) => glues_server::run(args).await,
        None => cli::run().await,
    }
}
