mod entry;
pub mod notebook;

use {
    crate::{logger::*, Action},
    ratatui::crossterm::event::KeyCode,
};
pub use {entry::EntryContext, notebook::NotebookContext};

pub enum ContextState {
    Entry,
    Notebook,
}

pub struct Context {
    pub entry: EntryContext,
    pub notebook: NotebookContext,

    pub state: ContextState,

    pub confirm: Option<(String, Action)>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            entry: EntryContext::default(),
            notebook: NotebookContext::default(),

            state: ContextState::Entry,
            confirm: None,
        }
    }
}

impl Context {
    pub fn consume(&mut self, code: KeyCode) -> Action {
        if self.confirm.is_some() {
            match code {
                KeyCode::Char('y') => {
                    let (_, action) = self.confirm.take().unwrap();
                    log("Context::consume - remove note!!!");
                    return action;
                }
                KeyCode::Char('n') => {
                    self.confirm = None;
                    return Action::None;
                }
                _ => return Action::None,
            }
        }

        match self.state {
            ContextState::Entry => self.entry.consume(code),
            ContextState::Notebook => self.notebook.consume(code),
        }
    }
}
