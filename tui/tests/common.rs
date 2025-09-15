#![allow(dead_code, async_fn_in_trait)]

#[path = "common/app_ext.rs"]
pub mod app_ext;
#[path = "common/terminal_ext.rs"]
pub mod terminal_ext;

pub use app_ext::AppTestExt;
#[allow(unused_imports)]
pub use terminal_ext::TerminalTestExt;

use color_eyre::Result;
use glues::{App, config, logger};
use ratatui::{Terminal, backend::TestBackend};
use std::string::String;

pub fn assert_snapshot(name: &str, lines: &[String]) {
    insta::assert_debug_snapshot!(name, lines);
}

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

// Build snapshot lines from the current terminal buffer.
pub fn snapshot_lines(term: &Terminal<TestBackend>) -> Vec<String> {
    let buf = term.backend().buffer().clone();
    let area = buf.area();
    let mut lines = Vec::with_capacity(area.height as usize);
    for y in 0..area.height {
        let mut line = String::new();
        for x in 0..area.width {
            line.push_str(buf[(x, y)].symbol());
        }
        lines.push(line);
    }
    lines
}
