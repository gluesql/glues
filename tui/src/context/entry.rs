use {
    crate::{
        action::{Action, OpenGitStep, OpenMongoStep, TuiAction},
        config::{
            self, LAST_CSV_PATH, LAST_FILE_PATH, LAST_GIT_PATH, LAST_JSON_PATH, LAST_MONGO_CONN_STR,
        },
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
pub const MONGO: &str = "[6] MongoDB";
pub const HELP: &str = "[h] Help";
pub const QUIT: &str = "[q] Quit";

pub const MENU_ITEMS: [&str; 8] = [INSTANT, CSV, JSON, FILE, GIT, MONGO, HELP, QUIT];

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
    pub async fn consume(&mut self, code: KeyCode) -> Action {
        let open = |key, action: TuiAction| async move {
            TuiAction::Prompt {
                message: vec![
                    Line::raw("Enter the path:"),
                    Line::from("If path not exists, it will be created.".dark_gray()),
                ],
                action: Box::new(action.into()),
                default: config::get(key).await,
            }
            .into()
        };

        let open_git = || async move {
            TuiAction::Prompt {
                message: vec![
                    Line::raw("Enter the git repository path:"),
                    Line::from("The path must contain an existing .git repository;".dark_gray()),
                    Line::from("otherwise, an error will occur.".dark_gray()),
                ],
                action: Box::new(TuiAction::OpenGit(OpenGitStep::Path).into()),
                default: config::get(LAST_GIT_PATH).await,
            }
            .into()
        };

        let open_mongo = || async move {
            TuiAction::Prompt {
                message: vec![
                    Line::raw("Enter the MongoDB connection string:"),
                    Line::from("e.g. mongodb://localhost:27017".dark_gray()),
                ],
                action: Box::new(TuiAction::OpenMongo(OpenMongoStep::ConnStr).into()),
                default: config::get(LAST_MONGO_CONN_STR).await,
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
            KeyCode::Char('2') => open(LAST_CSV_PATH, TuiAction::OpenCsv).await,
            KeyCode::Char('3') => open(LAST_JSON_PATH, TuiAction::OpenJson).await,
            KeyCode::Char('4') => open(LAST_FILE_PATH, TuiAction::OpenFile).await,
            KeyCode::Char('5') => open_git().await,
            KeyCode::Char('6') => open_mongo().await,
            KeyCode::Char('h') => TuiAction::Help.into(),

            KeyCode::Enter => {
                let i = self
                    .list_state
                    .selected()
                    .log_expect("EntryContext::consume: selected is None. This should not happen.");
                match MENU_ITEMS[i] {
                    INSTANT => EntryEvent::OpenMemory.into(),
                    CSV => open(LAST_CSV_PATH, TuiAction::OpenCsv).await,
                    JSON => open(LAST_JSON_PATH, TuiAction::OpenJson).await,
                    FILE => open(LAST_FILE_PATH, TuiAction::OpenFile).await,
                    GIT => open_git().await,
                    MONGO => open_mongo().await,
                    HELP => TuiAction::Help.into(),
                    QUIT => TuiAction::Quit.into(),
                    _ => Action::None,
                }
            }
            _ => Action::None,
        }
    }
}
