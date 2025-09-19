use tui_textarea::Input as TextAreaInput;

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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use ratzilla::event as rz;

    impl From<rz::KeyEvent> for KeyEvent {
        fn from(event: rz::KeyEvent) -> Self {
            let code = match event.code {
                rz::KeyCode::Char(c) => KeyCode::Char(c),
                rz::KeyCode::F(n) => KeyCode::F(n),
                rz::KeyCode::Backspace => KeyCode::Backspace,
                rz::KeyCode::Enter => KeyCode::Enter,
                rz::KeyCode::Left => KeyCode::Left,
                rz::KeyCode::Right => KeyCode::Right,
                rz::KeyCode::Up => KeyCode::Up,
                rz::KeyCode::Down => KeyCode::Down,
                rz::KeyCode::Tab => KeyCode::Tab,
                rz::KeyCode::Delete => KeyCode::Delete,
                rz::KeyCode::Home => KeyCode::Home,
                rz::KeyCode::End => KeyCode::End,
                rz::KeyCode::PageUp => KeyCode::PageUp,
                rz::KeyCode::PageDown => KeyCode::PageDown,
                rz::KeyCode::Esc => KeyCode::Esc,
                rz::KeyCode::Unidentified => KeyCode::Null,
            };

            KeyEvent {
                code,
                modifiers: KeyModifiers::new(event.ctrl, event.alt, event.shift),
                kind: KeyEventKind::Press,
            }
        }
    }
}

pub fn to_textarea_input(input: &Input) -> Option<TextAreaInput> {
    match input {
        Input::Key(key) => Some(key_to_textarea_input(key)),
        _ => None,
    }
}

pub fn key_to_textarea_input(key: &KeyEvent) -> TextAreaInput {
    use tui_textarea::Key;

    let key_code = match key.code {
        KeyCode::Char(c) => Key::Char(c),
        KeyCode::F(n) => Key::F(n),
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Enter => Key::Enter,
        KeyCode::Left => Key::Left,
        KeyCode::Right => Key::Right,
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::Tab => Key::Tab,
        KeyCode::Delete => Key::Delete,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::Esc => Key::Esc,
        KeyCode::Null => Key::Null,
    };

    TextAreaInput {
        key: key_code,
        ctrl: key.modifiers.ctrl,
        alt: key.modifiers.alt,
        shift: key.modifiers.shift,
    }
}
