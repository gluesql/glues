#[derive(Clone, Debug)]
pub enum Input {
    Key(KeyEvent),
    Paste(String),
    Resize(u16, u16),
}

#[derive(Clone, Debug)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    pub kind: KeyEventKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyCode {
    Char(char),
    F(u8),
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Tab,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Esc,
    Null,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyEventKind {
    Press,
    Release,
    Repeat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyModifiers {
    pub const NONE: Self = Self {
        ctrl: false,
        alt: false,
        shift: false,
    };

    pub const CONTROL: Self = Self {
        ctrl: true,
        alt: false,
        shift: false,
    };

    pub fn new(ctrl: bool, alt: bool, shift: bool) -> Self {
        Self { ctrl, alt, shift }
    }
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self::NONE
    }
}

mod native {
    use super::*;
    use ratatui::crossterm::event as ct;

    impl From<ct::Event> for Input {
        fn from(event: ct::Event) -> Self {
            match event {
                ct::Event::Key(key) => Input::Key(key.into()),
                ct::Event::Paste(paste) => Input::Paste(paste),
                ct::Event::Resize(w, h) => Input::Resize(w, h),
                _ => Input::Resize(0, 0),
            }
        }
    }

    impl From<ct::KeyEvent> for KeyEvent {
        fn from(key: ct::KeyEvent) -> Self {
            let modifiers = KeyModifiers::new(
                key.modifiers.contains(ct::KeyModifiers::CONTROL),
                key.modifiers.contains(ct::KeyModifiers::ALT),
                key.modifiers.contains(ct::KeyModifiers::SHIFT),
            );

            let code = match key.code {
                ct::KeyCode::Char(c) => KeyCode::Char(c),
                ct::KeyCode::F(n) => KeyCode::F(n),
                ct::KeyCode::Backspace => KeyCode::Backspace,
                ct::KeyCode::Enter => KeyCode::Enter,
                ct::KeyCode::Left => KeyCode::Left,
                ct::KeyCode::Right => KeyCode::Right,
                ct::KeyCode::Up => KeyCode::Up,
                ct::KeyCode::Down => KeyCode::Down,
                ct::KeyCode::Tab => KeyCode::Tab,
                ct::KeyCode::Delete => KeyCode::Delete,
                ct::KeyCode::Home => KeyCode::Home,
                ct::KeyCode::End => KeyCode::End,
                ct::KeyCode::PageUp => KeyCode::PageUp,
                ct::KeyCode::PageDown => KeyCode::PageDown,
                ct::KeyCode::Esc => KeyCode::Esc,
                _ => KeyCode::Null,
            };

            let kind = match key.kind {
                ct::KeyEventKind::Press => KeyEventKind::Press,
                ct::KeyEventKind::Release => KeyEventKind::Release,
                ct::KeyEventKind::Repeat => KeyEventKind::Repeat,
            };

            KeyEvent {
                code,
                modifiers,
                kind,
            }
        }
    }
}
