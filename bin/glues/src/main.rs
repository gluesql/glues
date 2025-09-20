use {
    clap::{Parser, Subcommand},
    color_eyre::Result,
    glues_server::ServerArgs,
    glues_tui::cli::{self, TuiArgs},
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
    #[command(flatten)]
    tui: TuiArgs,

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
    let Cli { tui, command } = Cli::parse();

    match command {
        Some(Command::Server(args)) => glues_server::run(args).await,
        None => cli::run(tui).await,
    }
}
