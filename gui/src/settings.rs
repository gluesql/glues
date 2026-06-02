use {
    serde::{Deserialize, Serialize},
    std::{
        env, fs,
        io::ErrorKind,
        path::{Path, PathBuf},
    },
};

const SETTINGS_FILE: &str = "gui-settings.json";
const MAX_RECENT_WORKSPACES: usize = 8;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GuiSettings {
    #[serde(default)]
    pub last_backend: Option<RecentBackend>,
    #[serde(default)]
    pub recent_workspaces: Vec<RecentWorkspace>,
}

impl GuiSettings {
    pub fn load() -> Result<Self, String> {
        let path = settings_path();
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Self::default()),
            Err(error) => {
                return Err(format!(
                    "Failed to read GUI settings from {}: {error}",
                    path.display()
                ));
            }
        };

        if content.trim().is_empty() {
            return Ok(Self::default());
        }

        serde_json::from_str(&content).map_err(|error| {
            format!(
                "Failed to parse GUI settings from {}: {error}",
                path.display()
            )
        })
    }

    pub fn save(&self) -> Result<(), String> {
        let path = settings_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "Failed to create GUI settings directory {}: {error}",
                    parent.display()
                )
            })?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|error| format!("Failed to encode GUI settings: {error}"))?;
        fs::write(&path, content).map_err(|error| {
            format!(
                "Failed to write GUI settings to {}: {error}",
                path.display()
            )
        })
    }

    pub fn upsert_recent(&mut self, recent: RecentWorkspace) {
        let key = recent.dedupe_key();
        self.recent_workspaces
            .retain(|item| item.dedupe_key() != key);
        self.recent_workspaces.insert(0, recent);
        self.recent_workspaces.truncate(MAX_RECENT_WORKSPACES);
    }

    pub fn first_recent_for_backend(&self, backend: RecentBackend) -> Option<&RecentWorkspace> {
        self.recent_workspaces
            .iter()
            .find(|recent| recent.backend() == backend)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RecentBackend {
    Memory,
    File,
    Redb,
    Git,
    Mongo,
    Proxy,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "backend", rename_all = "lowercase")]
pub enum RecentWorkspace {
    File {
        path: String,
    },
    Redb {
        path: String,
    },
    Git {
        path: String,
        remote: String,
        branch: String,
    },
    Mongo {
        conn_str: String,
        db_name: String,
    },
    Proxy {
        url: String,
    },
}

impl RecentWorkspace {
    pub fn backend(&self) -> RecentBackend {
        match self {
            Self::File { .. } => RecentBackend::File,
            Self::Redb { .. } => RecentBackend::Redb,
            Self::Git { .. } => RecentBackend::Git,
            Self::Mongo { .. } => RecentBackend::Mongo,
            Self::Proxy { .. } => RecentBackend::Proxy,
        }
    }

    pub fn backend_label(&self) -> &'static str {
        match self {
            Self::File { .. } => "File",
            Self::Redb { .. } => "redb",
            Self::Git { .. } => "Git",
            Self::Mongo { .. } => "Mongo",
            Self::Proxy { .. } => "Proxy",
        }
    }

    pub fn title(&self) -> String {
        match self {
            Self::File { path } | Self::Redb { path } => path_title(path),
            Self::Git { path, .. } => path_title(path),
            Self::Mongo { db_name, .. } => db_name.clone(),
            Self::Proxy { url } => proxy_title(url),
        }
    }

    pub fn subtitle(&self) -> String {
        match self {
            Self::File { path } | Self::Redb { path } => path.clone(),
            Self::Git {
                path,
                remote,
                branch,
            } => format!("{branch} - {remote} - {path}"),
            Self::Mongo { conn_str, .. } => conn_str.clone(),
            Self::Proxy { url } => url.clone(),
        }
    }

    fn dedupe_key(&self) -> String {
        match self {
            Self::File { path } => format!("file:{path}"),
            Self::Redb { path } => format!("redb:{path}"),
            Self::Git { path, .. } => format!("git:{path}"),
            Self::Mongo { conn_str, db_name } => format!("mongo:{conn_str}:{db_name}"),
            Self::Proxy { url } => format!("proxy:{url}"),
        }
    }
}

fn settings_path() -> PathBuf {
    settings_dir().join(SETTINGS_FILE)
}

fn settings_dir() -> PathBuf {
    if let Some(path) = env::var_os("GLUES_GUI_CONFIG_DIR") {
        return PathBuf::from(path);
    }

    let home = env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));

    if cfg!(target_os = "macos") {
        home.join("Library")
            .join("Application Support")
            .join("Glues")
    } else if let Some(path) = env::var_os("XDG_CONFIG_HOME") {
        PathBuf::from(path).join("glues")
    } else {
        home.join(".config").join("glues")
    }
}

fn path_title(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or(path)
        .to_owned()
}

fn proxy_title(url: &str) -> String {
    let without_scheme = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);

    without_scheme
        .split('/')
        .next()
        .filter(|host| !host.is_empty())
        .unwrap_or(url)
        .to_owned()
}
