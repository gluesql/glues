#![allow(dead_code, async_fn_in_trait)]

use color_eyre::Result;
use ratatui::{
    Terminal,
    backend::TestBackend,
    crossterm::event::{Event as Input, KeyCode, KeyEvent as CKeyEvent, KeyModifiers},
};

use glues::{App, config, logger};

// --- Internal helpers ------------------------------------------------------

async fn process_input(app: &mut App, input: Input) -> bool {
    use ratatui::crossterm::event::{
        Event as Input, KeyCode, KeyEvent as CKeyEvent, KeyEventKind, KeyModifiers,
    };

    if !matches!(
        input,
        Input::Key(CKeyEvent {
            kind: KeyEventKind::Press,
            ..
        })
    ) {
        return false;
    }

    match input {
        Input::Key(CKeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => true,
        _ => {
            let action = app.context_mut().consume(&input).await;
            app.handle_action(action, input).await
        }
    }
}

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

// --- Extension trait: ergonomic helpers on App ----------------------------
pub trait AppTestExt {
    fn draw_frame(&mut self, term: &mut Terminal<TestBackend>) -> color_eyre::Result<()>;
    async fn press(&mut self, c: char) -> bool;
    async fn ctrl(&mut self, c: char) -> bool;
    async fn key(&mut self, code: KeyCode) -> bool;
    async fn open_instant(&mut self, term: &mut Terminal<TestBackend>) -> Result<()>;
}

impl AppTestExt for App {
    fn draw_frame(&mut self, term: &mut Terminal<TestBackend>) -> color_eyre::Result<()> {
        term.draw(|f| self.draw(f))?;
        Ok(())
    }

    async fn press(&mut self, c: char) -> bool {
        process_input(
            self,
            Input::Key(CKeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)),
        )
        .await
    }

    async fn ctrl(&mut self, c: char) -> bool {
        process_input(
            self,
            Input::Key(CKeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)),
        )
        .await
    }

    async fn key(&mut self, code: KeyCode) -> bool {
        process_input(self, Input::Key(CKeyEvent::new(code, KeyModifiers::NONE))).await
    }
    async fn open_instant(&mut self, term: &mut Terminal<TestBackend>) -> Result<()> {
        term.draw(|f| self.draw(f))?;
        process_input(
            self,
            Input::Key(CKeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE)),
        )
        .await;
        term.draw(|f| self.draw(f))?;
        Ok(())
    }
}
