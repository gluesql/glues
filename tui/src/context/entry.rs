use {
    crate::{
        action::{Action, OpenGitStep, OpenMongoStep, TuiAction},
        config::{
            self, LAST_CSV_PATH, LAST_FILE_PATH, LAST_GIT_PATH, LAST_JSON_PATH,
            LAST_MONGO_CONN_STR, LAST_REMOTE_ADDR,
        },
        logger::*,
        theme::THEME,
    },
    glues_core::EntryEvent,
    ratatui::{crossterm::event::KeyCode, style::Stylize, text::Line, widgets::ListState},
};

pub const INSTANT: &str = "[1] Instant";
pub const FILE: &str = "[2] Local";
pub const GIT: &str = "[3] Git";
pub const MONGO: &str = "[4] MongoDB";
pub const CSV: &str = "[5] CSV";
pub const JSON: &str = "[6] JSON";
pub const REMOTE: &str = "[7] Remote";
pub const HELP: &str = "[h] Help";
pub const QUIT: &str = "[q] Quit";

pub const MENU_ITEMS: [&str; 9] = [INSTANT, FILE, GIT, MONGO, CSV, JSON, REMOTE, HELP, QUIT];

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
                    Line::from("If the path does not exist, it will be created.".fg(THEME.hint)),
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
                    Line::from("The path must contain an existing .git repository.".fg(THEME.hint)),
                    Line::from("otherwise, an error will occur.".fg(THEME.hint)),
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
                    Line::from("e.g. mongodb://localhost:27017".fg(THEME.hint)),
                ],
                action: Box::new(TuiAction::OpenMongo(OpenMongoStep::ConnStr).into()),
                default: config::get(LAST_MONGO_CONN_STR).await,
            }
            .into()
        };

        let open_remote = || async move {
            TuiAction::Prompt {
                message: vec![Line::raw("Enter the remote address:")],
                action: Box::new(TuiAction::OpenRemote.into()),
                default: config::get(LAST_REMOTE_ADDR).await,
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
            KeyCode::Char('2') => open(LAST_FILE_PATH, TuiAction::OpenFile).await,
            KeyCode::Char('3') => open_git().await,
            KeyCode::Char('4') => open_mongo().await,
            KeyCode::Char('5') => open(LAST_CSV_PATH, TuiAction::OpenCsv).await,
            KeyCode::Char('6') => open(LAST_JSON_PATH, TuiAction::OpenJson).await,
            KeyCode::Char('7') => open_remote().await,
            KeyCode::Char('a') => TuiAction::Help.into(),

            KeyCode::Enter => {
                let i = self
                    .list_state
                    .selected()
                    .log_expect("EntryContext::consume: selected is None. This should not happen.");
                match MENU_ITEMS[i] {
                    INSTANT => EntryEvent::OpenMemory.into(),
                    FILE => open(LAST_FILE_PATH, TuiAction::OpenFile).await,
                    GIT => open_git().await,
                    MONGO => open_mongo().await,
                    CSV => open(LAST_CSV_PATH, TuiAction::OpenCsv).await,
                    JSON => open(LAST_JSON_PATH, TuiAction::OpenJson).await,
                    REMOTE => open_remote().await,
                    HELP => TuiAction::Help.into(),
                    QUIT => TuiAction::Quit.into(),
                    _ => Action::None,
                }
            }
            _ => Action::PassThrough,
        }
    }
}
