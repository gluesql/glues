#[cfg(not(target_arch = "wasm32"))]
use glues_tui::cli;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    cli::run_cli().await
}

#[cfg(target_arch = "wasm32")]
fn main() {}
