use {
    crate::action::{Action, TuiAction},
    glues_core::EntryEvent,
    ratatui::{crossterm::event::KeyCode, widgets::ListState},
};

pub struct EntryContext {
    pub list_state: ListState,
}

impl EntryContext {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn consume(&mut self, code: KeyCode) -> Action {
        match code {
            KeyCode::Char('q') => return TuiAction::Quit.into(),
            KeyCode::Char('j') | KeyCode::Down => {
                self.list_state.select_next();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.list_state.select_previous();
            }
            KeyCode::Enter => return EntryEvent::OpenMemory.into(),
            _ => {}
        };

        Action::None
    }
}
