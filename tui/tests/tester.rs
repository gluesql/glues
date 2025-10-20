use {
    color_eyre::Result,
    glues_tui::{
        App, config,
        input::{Input, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
        logger,
    },
    once_cell::sync::Lazy,
    ratatui::{Terminal, backend::TestBackend},
    regex::Regex,
};

pub struct Tester {
    pub app: App,
    pub term: Terminal<TestBackend>,
}

fn key_press(code: KeyCode, modifiers: KeyModifiers) -> Input {
    Input::Key(KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
    })
}

// Bring concise snapshot macros into test files.
// Usage: snap!(t, "name") or snap_sanitized!(t, "name");
#[allow(unused_macros)]
macro_rules! snap {
    ($t:expr, $name:expr) => {{
        let text = $t.snapshot_text();
        insta::assert_snapshot!($name, text);
    }};
}

#[allow(unused_macros)]
macro_rules! snap_sanitized {
    ($t:expr, $name:expr) => {{
        let text = $t.snapshot_text_sanitized();
        insta::assert_snapshot!($name, text);
    }};
}

impl Tester {
    pub async fn new() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        let test_config_dir = cwd.join(".glues");
        std::fs::create_dir_all(&test_config_dir)?;
        // Set GLUES_CONFIG_DIR to isolate test config from user's actual config.
        // This prevents tests from polluting ~/.glues/ directory.
        unsafe {
            std::env::set_var("GLUES_CONFIG_DIR", &test_config_dir);
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
        self.handle_input(key_press(KeyCode::Char(c), KeyModifiers::NONE))
            .await
    }

    #[allow(dead_code)]
    pub async fn ctrl(&mut self, c: char) -> bool {
        self.handle_input(key_press(KeyCode::Char(c), KeyModifiers::CONTROL))
            .await
    }

    #[allow(dead_code)]
    pub async fn key(&mut self, code: KeyCode) -> bool {
        self.handle_input(key_press(code, KeyModifiers::NONE)).await
    }

    #[allow(dead_code)]
    pub async fn open_instant(&mut self) -> Result<()> {
        self.draw()?;
        let _ = self.press('i').await;
        self.draw()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn type_str(&mut self, s: &str) {
        for ch in s.chars() {
            let _ = self.press(ch).await;
        }
    }

    #[allow(dead_code)]
    pub async fn backspace(&mut self, n: usize) {
        for _ in 0..n {
            let _ = self.key(KeyCode::Backspace).await;
        }
    }

    #[allow(dead_code)]
    pub async fn open_first_note(&mut self) -> Result<()> {
        self.press('j').await;
        self.press('l').await;
        self.draw()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn snapshot_text(&self) -> String {
        buffer_lines(&self.term).join("\n")
    }

    #[allow(dead_code)]
    pub fn snapshot_text_sanitized(&self) -> String {
        sanitize_snapshot(&self.snapshot_text())
    }

    async fn handle_input(&mut self, input: Input) -> bool {
        if !matches!(
            input,
            Input::Key(KeyEvent {
                kind: KeyEventKind::Press,
                ..
            })
        ) {
            return false;
        }

        match input {
            Input::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            }) if modifiers.ctrl => true,
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

#[allow(dead_code)]
fn sanitize_snapshot(text: &str) -> String {
    static GAP_BEFORE_PIPE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ ]{2,}│$").unwrap());

    let mut sanitized = text.to_owned();

    static NOTE_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"(Note ID: )[0-9A-Fa-f-]+").unwrap());
    sanitized = NOTE_ID
        .replace_all(&sanitized, "${1}00000000-0000-0000-0000-000000000000")
        .into_owned();

    static DIRECTORY_ID: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(Directory ID: )[0-9A-Fa-f-]+").unwrap());
    sanitized = DIRECTORY_ID
        .replace_all(&sanitized, "${1}00000000-0000-0000-0000-000000000000")
        .into_owned();

    static PARENT_ID: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(Parent ID: )[0-9A-Fa-f-]+").unwrap());
    sanitized = PARENT_ID
        .replace_all(&sanitized, "${1}00000000-0000-0000-0000-000000000000")
        .into_owned();

    static CREATED_AT: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(Created at: )[0-9T:.-]+Z").unwrap());
    sanitized = CREATED_AT
        .replace_all(&sanitized, "${1}1970-01-01T00:00:00.000000Z")
        .into_owned();

    static UPDATED_AT: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(Updated at: )[0-9T:.-]+Z").unwrap());
    sanitized = UPDATED_AT
        .replace_all(&sanitized, "${1}1970-01-01T00:00:00.000000Z")
        .into_owned();

    // remove trailing spaces so snapshots stay stable across environments
    sanitized = sanitized
        .lines()
        .map(|line| {
            let trimmed = line.trim_end();
            GAP_BEFORE_PIPE.replace(trimmed, " │").into_owned()
        })
        .collect::<Vec<_>>()
        .join("\n");

    sanitized
}
