#![cfg(feature = "test-utils")]
#![allow(dead_code, async_fn_in_trait)]

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

// --- Input helpers ---------------------------------------------------------

pub async fn setup_app_and_term() -> Result<(App, Terminal<TestBackend>)> {
    // ensure logger/config have a writable HOME directory
    let cwd = std::env::current_dir()?;
    // ensure .glues directory exists to satisfy CsvStorage
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

pub async fn open_instant(app: &mut App, term: &mut Terminal<TestBackend>) -> Result<()> {
    term.draw(|f| app.draw(f))?;
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('1'),
        KeyModifiers::NONE,
    )))
    .await;
    term.draw(|f| app.draw(f))?;
    Ok(())
}

// --- Extension trait: ergonomic helpers on App ----------------------------
pub trait AppTestExt {
    fn draw_frame(&mut self, term: &mut Terminal<TestBackend>) -> color_eyre::Result<()>;
    async fn press(&mut self, c: char) -> bool;
    async fn ctrl(&mut self, c: char) -> bool;
    async fn key(&mut self, code: KeyCode) -> bool;
}

impl AppTestExt for App {
    fn draw_frame(&mut self, term: &mut Terminal<TestBackend>) -> color_eyre::Result<()> {
        term.draw(|f| self.draw(f))?;
        Ok(())
    }

    async fn press(&mut self, c: char) -> bool {
        self.handle_input(Input::Key(KeyEvent::new(
            KeyCode::Char(c),
            KeyModifiers::NONE,
        )))
        .await
    }

    async fn ctrl(&mut self, c: char) -> bool {
        self.handle_input(Input::Key(KeyEvent::new(
            KeyCode::Char(c),
            KeyModifiers::CONTROL,
        )))
        .await
    }

    async fn key(&mut self, code: KeyCode) -> bool {
        self.handle_input(Input::Key(KeyEvent::new(code, KeyModifiers::NONE)))
            .await
    }
}
