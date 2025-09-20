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

#[cfg(target_arch = "wasm32")]
use crate::config::LAST_IDB_NAMESPACE;

#[cfg(not(target_arch = "wasm32"))]
use crate::{
    action::{OpenGitStep, OpenMongoStep},
    config::{LAST_FILE_PATH, LAST_GIT_PATH, LAST_MONGO_CONN_STR, LAST_REDB_PATH},
};

pub const INSTANT: &str = "[i] Instant";
#[cfg(target_arch = "wasm32")]
pub const INDEXED_DB: &str = "[d] IndexedDB";
#[cfg(not(target_arch = "wasm32"))]
pub const FILE: &str = "[l] Local";
#[cfg(not(target_arch = "wasm32"))]
pub const REDB: &str = "[r] redb";
#[cfg(not(target_arch = "wasm32"))]
pub const GIT: &str = "[g] Git";
#[cfg(not(target_arch = "wasm32"))]
pub const MONGO: &str = "[m] MongoDB";
#[cfg(target_arch = "wasm32")]
pub const PROXY: &str = "[p] Proxy";
#[cfg(not(target_arch = "wasm32"))]
pub const PROXY: &str = "[p] Proxy";
pub const HELP: &str = "[h] Help";
pub const THEME_MENU: &str = "[t] Theme";
pub const QUIT: &str = "[q] Quit";

#[cfg(not(target_arch = "wasm32"))]
pub const MENU_ITEMS: [&str; 9] = [
    INSTANT, FILE, REDB, GIT, MONGO, PROXY, HELP, THEME_MENU, QUIT,
];
#[cfg(target_arch = "wasm32")]
pub const MENU_ITEMS: [&str; 5] = [INSTANT, INDEXED_DB, PROXY, HELP, THEME_MENU];

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
        let open_redb = || async move {
            TuiAction::Prompt {
                message: vec![
                    Line::raw("Provide the redb database path:"),
                    Line::from(
                        "Glues will create or reuse the single-file redb database.".fg(THEME.hint),
                    ),
                ],
                action: Box::new(TuiAction::OpenRedb.into()),
                default: config::get(LAST_REDB_PATH).await,
            }
            .into()
        };

        #[cfg(target_arch = "wasm32")]
        let open_indexed_db = || async move {
            TuiAction::Prompt {
                message: vec![
                    Line::raw("Enter the IndexedDB namespace:"),
                    Line::from("Leave empty to use the default namespace (glues).".fg(THEME.hint)),
                ],
                action: Box::new(TuiAction::OpenIndexedDb.into()),
                default: config::get(LAST_IDB_NAMESPACE).await,
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
            #[cfg(not(target_arch = "wasm32"))]
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
            #[cfg(target_arch = "wasm32")]
            KeyCode::Char('d') => open_indexed_db().await,
            #[cfg(not(target_arch = "wasm32"))]
            KeyCode::Char('l') => open(LAST_FILE_PATH, TuiAction::OpenFile).await,
            #[cfg(not(target_arch = "wasm32"))]
            KeyCode::Char('r') => open_redb().await,
            #[cfg(not(target_arch = "wasm32"))]
            KeyCode::Char('g') => open_git().await,
            #[cfg(not(target_arch = "wasm32"))]
            KeyCode::Char('m') => open_mongo().await,
            KeyCode::Char('p') => open_proxy().await,
            KeyCode::Char('h') => TuiAction::Help.into(),
            KeyCode::Char('t') => TuiAction::OpenThemeMenu.into(),

            KeyCode::Enter => {
                let i = self
                    .list_state
                    .selected()
                    .log_expect("EntryContext::consume: selected is None. This should not happen.");
                match MENU_ITEMS[i] {
                    INSTANT => EntryEvent::OpenMemory.into(),
                    #[cfg(target_arch = "wasm32")]
                    INDEXED_DB => open_indexed_db().await,
                    #[cfg(not(target_arch = "wasm32"))]
                    FILE => open(LAST_FILE_PATH, TuiAction::OpenFile).await,
                    #[cfg(not(target_arch = "wasm32"))]
                    REDB => open_redb().await,
                    #[cfg(not(target_arch = "wasm32"))]
                    GIT => open_git().await,
                    #[cfg(not(target_arch = "wasm32"))]
                    MONGO => open_mongo().await,
                    PROXY => open_proxy().await,
                    HELP => TuiAction::Help.into(),
                    THEME_MENU => TuiAction::OpenThemeMenu.into(),
                    #[cfg(not(target_arch = "wasm32"))]
                    QUIT => TuiAction::Quit.into(),
                    _ => Action::None,
                }
            }
            _ => Action::PassThrough,
        }
    }
}
