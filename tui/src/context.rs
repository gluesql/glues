pub mod entry;
pub mod notebook;
pub mod theme_selector;

use theme_selector::ThemeSelector;
use {
    crate::{
        Action,
        config::{self, LAST_THEME},
        input::{Input, KeyCode, KeyEvent, to_textarea_input},
        log,
        logger::*,
        theme::{self, THEME},
    },
    glues_core::transition::VimKeymapKind,
    ratatui::{
        style::Style,
        text::Line,
        widgets::{Block, Borders},
    },
    std::time::SystemTime,
    tui_textarea::TextArea,
};
pub use {entry::EntryContext, notebook::NotebookContext};

pub enum ContextState {
    Entry,
    Notebook,
}

pub struct ContextPrompt {
    pub widget: TextArea<'static>,
    pub message: Vec<Line<'static>>,
    pub action: Action,
}

impl ContextPrompt {
    pub fn new(message: Vec<Line<'static>>, action: Action, default: Option<String>) -> Self {
        Self::with_mask(message, action, default, None)
    }

    pub fn new_masked(
        message: Vec<Line<'static>>,
        action: Action,
        default: Option<String>,
        mask_char: char,
    ) -> Self {
        Self::with_mask(message, action, default, Some(mask_char))
    }

    fn with_mask(
        message: Vec<Line<'static>>,
        action: Action,
        default: Option<String>,
        mask: Option<char>,
    ) -> Self {
        let mut widget = TextArea::new(vec![default.unwrap_or_default()]);
        if let Some(mask_char) = mask {
            widget.set_mask_char(mask_char);
        }
        widget.set_cursor_style(Style::default().fg(THEME.accent_text).bg(THEME.accent));
        widget.set_block(
            Block::default()
                .border_style(Style::default())
                .borders(Borders::ALL),
        );
        Self {
            widget,
            message,
            action,
        }
    }
}

pub struct QuitMenu {
    pub message: String,
    pub quit_action: Action,
    pub menu_action: Action,
}

impl QuitMenu {
    pub fn new(message: impl Into<String>, quit_action: Action, menu_action: Action) -> Self {
        Self {
            message: message.into(),
            quit_action,
            menu_action,
        }
    }
}

pub struct Context {
    pub entry: EntryContext,
    pub notebook: NotebookContext,

    pub state: ContextState,

    pub quit_menu: Option<QuitMenu>,
    pub confirm: Option<(String, Action)>,
    pub alert: Option<String>,
    pub prompt: Option<ContextPrompt>,
    pub theme_selector: Option<ThemeSelector>,
    pub last_log: Option<(String, SystemTime)>,

    pub help: bool,
    pub editor_keymap: bool,
    pub vim_keymap: Option<VimKeymapKind>,

    pub keymap: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            entry: EntryContext::default(),
            notebook: NotebookContext::default(),

            state: ContextState::Entry,
            quit_menu: None,
            confirm: None,
            alert: None,
            prompt: None,
            theme_selector: None,
            last_log: None,

            help: false,
            editor_keymap: false,
            vim_keymap: None,

            keymap: false,
        }
    }
}

impl Context {
    pub fn take_prompt_input(&mut self) -> Option<String> {
        self.prompt
            .take()?
            .widget
            .lines()
            .first()
            .map(ToOwned::to_owned)
    }

    pub async fn consume(&mut self, input: &Input) -> Action {
        if self.vim_keymap.is_some() {
            self.vim_keymap = None;
            return Action::None;
        } else if self.editor_keymap {
            self.editor_keymap = false;
            return Action::None;
        } else if self.help {
            self.help = false;
            return Action::None;
        } else if self.alert.is_some() {
            // any key pressed will close the alert
            self.alert = None;
            return Action::None;
        } else if self.quit_menu.is_some() {
            let code = match input {
                Input::Key(key) => key.code,
                _ => return Action::None,
            };

            match code {
                #[cfg(not(target_arch = "wasm32"))]
                KeyCode::Char('q') => {
                    let menu = self.quit_menu.take().log_expect("quit menu must be some");
                    return menu.quit_action;
                }
                KeyCode::Char('m') => {
                    let menu = self.quit_menu.take().log_expect("quit menu must be some");
                    return menu.menu_action;
                }
                KeyCode::Esc => {
                    self.quit_menu = None;
                    return Action::None;
                }
                _ => return Action::None,
            }
        } else if self.confirm.is_some() {
            let code = match input {
                Input::Key(key) => key.code,
                _ => return Action::None,
            };

            match code {
                KeyCode::Char('y') => {
                    let (_, action) = self.confirm.take().log_expect("confirm must be some");
                    log!("Context::consume - remove note!!!");
                    return action;
                }
                KeyCode::Char('n') => {
                    self.confirm = None;
                    return Action::None;
                }
                _ => return Action::None,
            }
        } else if let Some(selector) = self.theme_selector.as_mut() {
            let key = match input {
                Input::Key(key) => key,
                _ => return Action::None,
            };

            match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    selector.select_next();
                    return Action::None;
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    selector.select_previous();
                    return Action::None;
                }
                KeyCode::Enter => {
                    let preset = selector.selected();
                    theme::set_theme(preset.id);
                    config::update(LAST_THEME, preset.id.as_str()).await;
                    self.theme_selector = None;
                    return Action::None;
                }
                KeyCode::Esc => {
                    self.theme_selector = None;
                    return Action::None;
                }
                KeyCode::Char(char) => {
                    if let Some(preset) = selector.select_by_key(char) {
                        theme::set_theme(preset.id);
                        config::update(LAST_THEME, preset.id.as_str()).await;
                        self.theme_selector = None;
                    }
                    return Action::None;
                }
                _ => return Action::None,
            }
        } else if let Some(prompt) = self.prompt.as_ref() {
            match input {
                Input::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    return prompt.action.clone();
                }
                Input::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => {
                    self.prompt = None;
                    return Action::None;
                }
                _ => {
                    if let Some(text_input) = to_textarea_input(input) {
                        self.prompt
                            .as_mut()
                            .log_expect("prompt must be some")
                            .widget
                            .input(text_input);
                    }

                    return Action::None;
                }
            }
        }

        match self.state {
            ContextState::Entry => match input {
                Input::Key(key) => self.entry.consume(key.code).await,
                _ => Action::None,
            },
            ContextState::Notebook => self.notebook.consume(input),
        }
    }
}
