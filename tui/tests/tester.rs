use color_eyre::Result;
use glues::{App, config, logger};
use ratatui::{
    Terminal,
    backend::TestBackend,
    crossterm::event::{
        Event as Input, KeyCode, KeyEvent as CKeyEvent, KeyEventKind, KeyModifiers,
    },
};

pub struct Tester {
    pub app: App,
    pub term: Terminal<TestBackend>,
}

// Bring a concise snapshot macro into test files.
// Usage: snap!(t, "name");
#[allow(unused_macros)]
macro_rules! snap {
    ($t:expr, $name:expr) => {{
        let text = $t.snapshot_text();
        insta::assert_snapshot!($name, text);
    }};
}

impl Tester {
    pub async fn new() -> Result<Self> {
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
        Ok(Self { app, term })
    }

    pub fn draw(&mut self) -> color_eyre::Result<()> {
        self.term.draw(|f| self.app.draw(f))?;
        Ok(())
    }

    pub async fn press(&mut self, c: char) -> bool {
        self.handle_input(Input::Key(CKeyEvent::new(
            KeyCode::Char(c),
            KeyModifiers::NONE,
        )))
        .await
    }

    #[allow(dead_code)]
    pub async fn ctrl(&mut self, c: char) -> bool {
        self.handle_input(Input::Key(CKeyEvent::new(
            KeyCode::Char(c),
            KeyModifiers::CONTROL,
        )))
        .await
    }

    #[allow(dead_code)]
    pub async fn key(&mut self, code: KeyCode) -> bool {
        self.handle_input(Input::Key(CKeyEvent::new(code, KeyModifiers::NONE)))
            .await
    }

    #[allow(dead_code)]
    pub async fn open_instant(&mut self) -> Result<()> {
        self.draw()?;
        let _ = self.press('1').await;
        self.draw()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn snapshot_text(&self) -> String {
        buffer_lines(&self.term).join("\n")
    }

    async fn handle_input(&mut self, input: Input) -> bool {
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
                let action = self.app.context_mut().consume(&input).await;
                self.app.handle_action(action, input).await
            }
        }
    }
}

fn buffer_lines(term: &Terminal<TestBackend>) -> Vec<String> {
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
