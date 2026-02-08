use glues_tui::cli;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    cli::run().await
}
