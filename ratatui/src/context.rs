pub mod entry;
pub mod notebook;

use {
    crate::{logger::*, Action},
    ratatui::{
        crossterm::event::{Event as Input, KeyCode, KeyEvent},
        style::{Style, Stylize},
        text::Line,
        widgets::{Block, Borders},
    },
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
        let mut widget = TextArea::new(vec![default.unwrap_or_default()]);
        widget.set_cursor_style(Style::default().white().on_blue());
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
    pub help: bool,
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
            help: false,
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

    pub fn consume(&mut self, input: &Input) -> Action {
        if self.help {
            // any key pressed will close the help dialog
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
                    log("Context::consume - remove note!!!");
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
        }

        let code = match input {
            Input::Key(key) => key.code,
            _ => return Action::None,
        };

        match self.state {
            ContextState::Entry => self.entry.consume(code),
            ContextState::Notebook => self.notebook.consume(code),
        }
    }
}
