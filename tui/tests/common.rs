#![cfg(feature = "test-utils")]

use color_eyre::Result;
use ratatui::{
    Terminal,
    backend::TestBackend,
    crossterm::event::{Event as Input, KeyCode, KeyEvent, KeyModifiers},
};

use glues::{App, config, logger};

pub fn buffer_to_lines(term: &Terminal<TestBackend>) -> Vec<String> {
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

pub async fn setup_app_and_term() -> Result<(App, Terminal<TestBackend>)> {
    // ensure logger/config have a writable HOME directory
    let cwd = std::env::current_dir()?;
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

pub async fn open_instant(app: &mut App, term: &mut Terminal<TestBackend>) -> Result<()> {
    app.draw_once_on(term)?;
    app
        .handle_input(Input::Key(KeyEvent::new(
            KeyCode::Char('1'),
            KeyModifiers::NONE,
        )))
        .await;
    app.draw_once_on(term)?;
    Ok(())
}

