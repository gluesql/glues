use {
    crate::{
        action::{Action, TuiAction},
        config::{self, LAST_CSV_PATH, LAST_FILE_PATH, LAST_JSON_PATH},
        logger::*,
    },
    glues_core::EntryEvent,
    ratatui::{crossterm::event::KeyCode, style::Stylize, text::Line, widgets::ListState},
};

pub const INSTANT: &str = "[1] Instant";
pub const CSV: &str = "[2] CSV";
pub const JSON: &str = "[3] JSON";
pub const FILE: &str = "[4] File";
pub const GIT: &str = "[5] Git";
pub const QUIT: &str = "[q] Quit";

pub const MENU_ITEMS: [&str; 6] = [INSTANT, CSV, JSON, FILE, GIT, QUIT];

pub struct EntryContext {
    pub list_state: ListState,
}

impl Default for EntryContext {
    fn default() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(0)),
        }
    }
}

impl EntryContext {
    pub fn consume(&mut self, code: KeyCode) -> Action {
        let open = |key, action: TuiAction| {
            TuiAction::Prompt {
                message: vec![
                    Line::raw("Enter the path:"),
                    Line::from("If path not exists, it will be created.".dark_gray()),
                ],
                action: Box::new(action.into()),
                default: config::get(key),
            }
            .into()
        };

        match code {
            KeyCode::Char('q') => TuiAction::Quit.into(),
            KeyCode::Char('j') | KeyCode::Down => {
                self.list_state.select_next();
                Action::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.list_state.select_previous();
                Action::None
            }
            KeyCode::Char('1') => EntryEvent::OpenMemory.into(),
            KeyCode::Char('2') => open(LAST_CSV_PATH, TuiAction::OpenCsv),
            KeyCode::Char('3') => open(LAST_JSON_PATH, TuiAction::OpenJson),
            KeyCode::Char('4') => open(LAST_FILE_PATH, TuiAction::OpenFile),
            KeyCode::Enter => {
                let i = self
                    .list_state
                    .selected()
                    .log_expect("EntryContext::consume: selected is None. This should not happen.");
                match MENU_ITEMS[i] {
                    INSTANT => EntryEvent::OpenMemory.into(),
                    CSV => open(LAST_CSV_PATH, TuiAction::OpenCsv),
                    JSON => open(LAST_JSON_PATH, TuiAction::OpenJson),
                    FILE => open(LAST_FILE_PATH, TuiAction::OpenFile),
                    GIT => TuiAction::Alert("Not implemented yet.".to_string()).into(),
                    QUIT => TuiAction::Quit.into(),
                    _ => Action::None,
                }
            }
            _ => Action::None,
        }
    }
}
