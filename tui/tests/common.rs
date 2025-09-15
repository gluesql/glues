#![allow(dead_code, async_fn_in_trait)]

#[path = "common/app_ext.rs"]
pub mod app_ext;
#[path = "common/terminal_ext.rs"]
pub mod terminal_ext;

pub use app_ext::AppTestExt;
pub use terminal_ext::TerminalTestExt;

use color_eyre::Result;
use glues::{App, config, logger};
use ratatui::{Terminal, backend::TestBackend};

pub async fn setup_app_and_term() -> Result<(App, Terminal<TestBackend>)> {
    // Use repo cwd as HOME and ensure `.glues` exists; logger init is idempotent now
    let cwd = std::env::current_dir()?;
    std::fs::create_dir_all(cwd.join(".glues"))?;
    unsafe {
        std::env::set_var("HOME", &cwd);
    }
    config::init().await;
    logger::init().await;

    let backend = TestBackend::new(120, 40);
    let term = Terminal::new(backend)?;
    let app = App::new();

    Ok((app, term))
}
