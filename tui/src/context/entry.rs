use {
    crate::{
        action::{Action, TuiAction},
        config::{self, LAST_PROXY_URL},
        input::KeyCode,
        logger::*,
        theme::THEME,
    },
    glues_core::EntryEvent,
    ratatui::{style::Stylize, text::Line, widgets::ListState},
};

#[cfg(not(target_arch = "wasm32"))]
use crate::{
    action::{OpenGitStep, OpenMongoStep},
    config::{LAST_FILE_PATH, LAST_GIT_PATH, LAST_MONGO_CONN_STR},
};

pub const INSTANT: &str = "[i] Instant";
#[cfg(not(target_arch = "wasm32"))]
pub const FILE: &str = "[l] Local";
#[cfg(not(target_arch = "wasm32"))]
pub const GIT: &str = "[g] Git";
#[cfg(not(target_arch = "wasm32"))]
pub const MONGO: &str = "[m] MongoDB";
#[cfg(target_arch = "wasm32")]
pub const PROXY: &str = "[p] Proxy";
#[cfg(not(target_arch = "wasm32"))]
pub const PROXY: &str = "[p] Proxy";
pub const HELP: &str = "[h] Help";
pub const QUIT: &str = "[q] Quit";

#[cfg(not(target_arch = "wasm32"))]
pub const MENU_ITEMS: [&str; 7] = [INSTANT, FILE, GIT, MONGO, PROXY, HELP, QUIT];
#[cfg(target_arch = "wasm32")]
pub const MENU_ITEMS: [&str; 4] = [INSTANT, PROXY, HELP, QUIT];

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
        #[cfg(not(target_arch = "wasm32"))]
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

        #[cfg(not(target_arch = "wasm32"))]
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

        #[cfg(not(target_arch = "wasm32"))]
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

        let open_proxy = || async move {
            TuiAction::Prompt {
                message: vec![
                    Line::raw("Enter the proxy server URL:"),
                    Line::from("e.g. http://127.0.0.1:4000".fg(THEME.hint)),
                ],
                action: Box::new(TuiAction::OpenProxy.into()),
                default: config::get(LAST_PROXY_URL).await,
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
            KeyCode::Char('i') => EntryEvent::OpenMemory.into(),
            #[cfg(not(target_arch = "wasm32"))]
            KeyCode::Char('l') => open(LAST_FILE_PATH, TuiAction::OpenFile).await,
            #[cfg(not(target_arch = "wasm32"))]
            KeyCode::Char('g') => open_git().await,
            #[cfg(not(target_arch = "wasm32"))]
            KeyCode::Char('m') => open_mongo().await,
            KeyCode::Char('p') => open_proxy().await,
            KeyCode::Char('h') => TuiAction::Help.into(),

            KeyCode::Enter => {
                let i = self
                    .list_state
                    .selected()
                    .log_expect("EntryContext::consume: selected is None. This should not happen.");
                match MENU_ITEMS[i] {
                    INSTANT => EntryEvent::OpenMemory.into(),
                    #[cfg(not(target_arch = "wasm32"))]
                    FILE => open(LAST_FILE_PATH, TuiAction::OpenFile).await,
                    #[cfg(not(target_arch = "wasm32"))]
                    GIT => open_git().await,
                    #[cfg(not(target_arch = "wasm32"))]
                    MONGO => open_mongo().await,
                    PROXY => open_proxy().await,
                    HELP => TuiAction::Help.into(),
                    QUIT => TuiAction::Quit.into(),
                    _ => Action::None,
                }
            }
            _ => Action::PassThrough,
        }
    }
}
