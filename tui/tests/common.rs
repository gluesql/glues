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

pub fn draw_once(app: &mut App, term: &mut Terminal<TestBackend>) -> color_eyre::Result<()> {
    term.draw(|f| app.draw(f))?;
    Ok(())
}

// --- Input helpers ---------------------------------------------------------
pub fn ev_char(c: char) -> Input {
    Input::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE))
}

pub fn ev_ctrl(c: char) -> Input {
    Input::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL))
}

pub fn ev(code: KeyCode) -> Input {
    Input::Key(KeyEvent::new(code, KeyModifiers::NONE))
}

pub async fn send_char(app: &mut App, c: char) -> bool {
    app.handle_input(ev_char(c)).await
}

pub async fn send_ctrl(app: &mut App, c: char) -> bool {
    app.handle_input(ev_ctrl(c)).await
}

pub async fn send_code(app: &mut App, code: KeyCode) -> bool {
    app.handle_input(ev(code)).await
}

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
    app.draw_once_on(term)?;
    send_char(app, '1').await;
    app.draw_once_on(term)?;
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
        draw_once(self, term)
    }

    async fn press(&mut self, c: char) -> bool {
        send_char(self, c).await
    }

    async fn ctrl(&mut self, c: char) -> bool {
        send_ctrl(self, c).await
    }

    async fn key(&mut self, code: KeyCode) -> bool {
        send_code(self, code).await
    }
}
