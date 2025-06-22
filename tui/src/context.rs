pub mod entry;
pub mod notebook;

use {
    crate::{Action, action::TuiAction, log, logger::*, theme},
    glues_core::transition::VimKeymapKind,
    ratatui::{
        crossterm::event::{Event as Input, KeyCode, KeyEvent},
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
        let theme = theme::current_theme();
        let mut widget = TextArea::new(vec![default.unwrap_or_default()]);
        widget.set_cursor_style(Style::default().fg(theme.accent_text).bg(theme.accent));
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

pub struct Context {
    pub entry: EntryContext,
    pub notebook: NotebookContext,

    pub state: ContextState,

    pub confirm: Option<(String, Action)>,
    pub alert: Option<String>,
    pub prompt: Option<ContextPrompt>,
    pub last_log: Option<(String, SystemTime)>,

    pub help: bool,
    pub editor_keymap: bool,
    pub vim_keymap: Option<VimKeymapKind>,

    pub theme_dialog: bool,

    pub keymap: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            entry: EntryContext::default(),
            notebook: NotebookContext::default(),

            state: ContextState::Entry,
            confirm: None,
            alert: None,
            prompt: None,
            last_log: None,

            help: false,
            editor_keymap: false,
            vim_keymap: None,

            theme_dialog: false,

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
                    self.prompt
                        .as_mut()
                        .log_expect("prompt must be some")
                        .widget
                        .input(input.clone());

                    return Action::None;
                }
            }
        } else if self.theme_dialog {
            let code = match input {
                Input::Key(key) => key.code,
                _ => return Action::None,
            };

            if code == KeyCode::Esc {
                self.theme_dialog = false;
                return Action::None;
            }

            let action = self.entry.consume_theme(code);
            if matches!(action, Action::Tui(TuiAction::ChangeTheme(..))) {
                self.theme_dialog = false;
            }
            return action;
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
