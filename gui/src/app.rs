use {
    crate::{
        model::{
            OpenedNote, TreeNode, TreeRowKind, WorkspaceBackendConfig as BackendConfig,
            WorkspaceModel,
        },
        settings::{GuiSettings, RecentBackend, RecentWorkspace},
    },
    gpui::{
        App, AppContext, Application, Bounds, Context, Entity, InteractiveElement as _,
        IntoElement, KeyBinding, Menu, MenuItem, ParentElement, PathPromptOptions, Render,
        StatefulInteractiveElement as _, Styled, Subscription, SystemMenuType, Window,
        WindowBounds, WindowOptions, actions, div, prelude::FluentBuilder, px, size,
    },
    gpui_component::{
        ActiveTheme, Disableable, Icon, IconName, Root, Selectable, Sizable, StyledExt, Theme,
        alert::Alert,
        breadcrumb::Breadcrumb,
        button::{Button, ButtonGroup, ButtonVariants},
        form::{Field, Form},
        group_box::{GroupBox, GroupBoxVariants},
        h_flex,
        input::{Input, InputEvent, InputState},
        label::Label,
        list::ListItem,
        scroll::ScrollableElement,
        tab::{Tab, TabBar},
        tag::Tag,
        tree::{TreeItem, TreeState, tree},
        v_flex,
    },
    std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc},
    tokio::runtime::Runtime,
};

const DIRECTORY_ITEM_PREFIX: &str = "dir:";
const NOTE_ITEM_PREFIX: &str = "note:";
const LOADING_ITEM_PREFIX: &str = "loading:";

actions!(glues_gui, [Quit]);

#[derive(Clone)]
struct DraggedNoteTab {
    note_id: String,
    label: String,
}

impl Render for DraggedNoteTab {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .id("dragged-note-tab")
            .gap_2()
            .items_center()
            .px_3()
            .h(px(30.))
            .max_w(px(240.))
            .overflow_hidden()
            .border_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().tab_active)
            .text_color(cx.theme().tab_active_foreground)
            .shadow_md()
            .child(Icon::new(IconName::File).small())
            .child(Label::new(self.label.clone()).truncate())
    }
}

pub fn run() {
    application()
        .with_assets(gpui_component_assets::Assets)
        .run(|cx| {
            init(cx);

            let bounds = Bounds::centered(None, size(px(1200.), px(760.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                build_root,
            )
            .expect("failed to open Glues GUI window");

            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();

            cx.activate(true);
        });
}

#[cfg(target_os = "macos")]
fn application() -> Application {
    Application::with_platform(Rc::new(gpui_macos::MacPlatform::new(false)))
}

pub fn init(cx: &mut App) {
    gpui_component::init(cx);
    install_font_defaults(cx);
    install_app_bindings(cx);
}

pub fn build_root(window: &mut Window, cx: &mut App) -> Entity<Root> {
    let view = cx.new(|cx| GluesGui::new(window, cx));
    build_root_for_view(view, window, cx)
}

#[cfg(feature = "visual-tests")]
pub fn build_visual_demo_root(window: &mut Window, cx: &mut App) -> Entity<Root> {
    build_visual_workspace_root(
        window,
        cx,
        load_visual_workspace(WorkspaceModel::visual_demo()),
    )
}

#[cfg(feature = "visual-tests")]
pub fn build_visual_empty_viewer_root(window: &mut Window, cx: &mut App) -> Entity<Root> {
    build_visual_workspace_root(
        window,
        cx,
        load_visual_workspace(WorkspaceModel::visual_empty_viewer()),
    )
}

#[cfg(feature = "visual-tests")]
pub fn build_visual_long_note_root(window: &mut Window, cx: &mut App) -> Entity<Root> {
    build_visual_workspace_root(
        window,
        cx,
        load_visual_workspace(WorkspaceModel::visual_long_note()),
    )
}

#[cfg(feature = "visual-tests")]
pub fn build_visual_open_screen_root(window: &mut Window, cx: &mut App) -> Entity<Root> {
    let view =
        cx.new(|cx| GluesGui::new_with_settings(window, cx, visual_recent_settings(), None, false));
    build_root_for_view(view, window, cx)
}

#[cfg(feature = "visual-tests")]
fn build_visual_workspace_root(
    window: &mut Window,
    cx: &mut App,
    workspace_result: Result<WorkspaceModel, String>,
) -> Entity<Root> {
    let view = cx.new(|cx| {
        let mut this = GluesGui::new_with_settings(window, cx, GuiSettings::default(), None, false);

        match workspace_result {
            Ok(workspace) => {
                this.workspace = Some(workspace);
                this.sync_tree_state(cx);
            }
            Err(error) => this.error = Some(error),
        }

        this
    });
    build_root_for_view(view, window, cx)
}

#[cfg(feature = "visual-tests")]
fn load_visual_workspace(
    workspace: impl std::future::Future<Output = crate::model::Result<WorkspaceModel>>,
) -> Result<WorkspaceModel, String> {
    Runtime::new()
        .map_err(|err| err.to_string())
        .and_then(|runtime| runtime.block_on(workspace).map_err(|err| err.to_string()))
}

#[cfg(feature = "visual-tests")]
fn visual_recent_settings() -> GuiSettings {
    GuiSettings {
        last_backend: Some(RecentBackend::File),
        recent_workspaces: vec![
            RecentWorkspace::File {
                path: "/Users/dev/Notes/glues-file".to_owned(),
            },
            RecentWorkspace::Redb {
                path: "/Users/dev/Notes/archive.redb".to_owned(),
            },
            RecentWorkspace::Git {
                path: "/Users/dev/Workspace/glues-notes".to_owned(),
                remote: "git@github.com:example/notes.git".to_owned(),
                branch: "main".to_owned(),
            },
            RecentWorkspace::Proxy {
                url: "https://notes.example.test".to_owned(),
            },
        ],
    }
}

fn build_root_for_view(view: Entity<GluesGui>, window: &mut Window, cx: &mut App) -> Entity<Root> {
    cx.new(|cx| Root::new(view, window, cx))
}

fn install_app_bindings(cx: &mut App) {
    cx.on_action(|_: &Quit, cx| cx.quit());
    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    cx.set_menus(vec![Menu::new("Glues").items([
        MenuItem::os_submenu("Services", SystemMenuType::Services),
        MenuItem::separator(),
        MenuItem::action("Quit Glues", Quit),
    ])]);
}

fn install_font_defaults(cx: &mut App) {
    let theme = Theme::global_mut(cx);
    theme.font_family = theme.mono_font_family.clone();
    theme.font_size = theme.mono_font_size;

    let highlight_theme = std::sync::Arc::make_mut(&mut theme.highlight_theme);
    highlight_theme.style.editor_active_line = None;
    highlight_theme.style.editor_active_line_number = None;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BackendKind {
    Memory,
    File,
    Redb,
    Git,
    Mongo,
    Proxy,
}

impl BackendKind {
    const ALL: [Self; 6] = [
        Self::Memory,
        Self::File,
        Self::Redb,
        Self::Git,
        Self::Mongo,
        Self::Proxy,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Memory => "Memory",
            Self::File => "File",
            Self::Redb => "redb",
            Self::Git => "Git",
            Self::Mongo => "Mongo",
            Self::Proxy => "Proxy",
        }
    }

    fn key(self) -> &'static str {
        match self {
            Self::Memory => "memory",
            Self::File => "file",
            Self::Redb => "redb",
            Self::Git => "git",
            Self::Mongo => "mongo",
            Self::Proxy => "proxy",
        }
    }

    fn to_recent_backend(self) -> RecentBackend {
        match self {
            Self::Memory => RecentBackend::Memory,
            Self::File => RecentBackend::File,
            Self::Redb => RecentBackend::Redb,
            Self::Git => RecentBackend::Git,
            Self::Mongo => RecentBackend::Mongo,
            Self::Proxy => RecentBackend::Proxy,
        }
    }

    fn from_recent_backend(backend: RecentBackend) -> Self {
        match backend {
            RecentBackend::Memory => Self::Memory,
            RecentBackend::File => Self::File,
            RecentBackend::Redb => Self::Redb,
            RecentBackend::Git => Self::Git,
            RecentBackend::Mongo => Self::Mongo,
            RecentBackend::Proxy => Self::Proxy,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PathTarget {
    FileDirectory,
    RedbFile,
    GitDirectory,
}

impl PathTarget {
    fn button_id(self) -> &'static str {
        match self {
            Self::FileDirectory => "choose-file-storage-directory",
            Self::RedbFile => "choose-redb-file",
            Self::GitDirectory => "choose-git-directory",
        }
    }

    fn choose_label(self) -> &'static str {
        match self {
            Self::FileDirectory | Self::GitDirectory => "Choose folder",
            Self::RedbFile => "Choose file",
        }
    }

    fn change_label(self) -> &'static str {
        match self {
            Self::FileDirectory | Self::GitDirectory => "Change folder",
            Self::RedbFile => "Change file",
        }
    }

    fn empty_label(self) -> &'static str {
        match self {
            Self::FileDirectory | Self::GitDirectory => "No folder selected",
            Self::RedbFile => "No file selected",
        }
    }

    fn empty_description(self) -> &'static str {
        match self {
            Self::FileDirectory | Self::GitDirectory => "Choose a folder to continue.",
            Self::RedbFile => "Choose a file to continue.",
        }
    }

    fn icon(self) -> IconName {
        match self {
            Self::FileDirectory | Self::GitDirectory => IconName::FolderOpen,
            Self::RedbFile => IconName::File,
        }
    }

    fn options(self) -> PathPromptOptions {
        let directories = matches!(self, Self::FileDirectory | Self::GitDirectory);
        PathPromptOptions {
            files: !directories,
            directories,
            multiple: false,
            prompt: Some(self.choose_label().into()),
        }
    }
}

pub struct GluesGui {
    backend_kind: BackendKind,
    settings: GuiSettings,
    workspace: Option<WorkspaceModel>,
    note_editors: RefCell<HashMap<String, Entity<InputState>>>,
    tree_state: Entity<TreeState>,
    loading: bool,
    error: Option<String>,
    file_path: Option<PathBuf>,
    redb_path: Option<PathBuf>,
    git_path: Option<PathBuf>,
    git_remote: Entity<InputState>,
    git_branch: Entity<InputState>,
    mongo_conn: Entity<InputState>,
    mongo_db: Entity<InputState>,
    proxy_url: Entity<InputState>,
    proxy_token: Entity<InputState>,
    _input_subscriptions: Vec<Subscription>,
}

impl GluesGui {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::new_with_demo(
            window,
            cx,
            std::env::var_os("GLUES_GUI_VISUAL_DEMO").is_some(),
        )
    }

    fn new_with_demo(window: &mut Window, cx: &mut Context<Self>, open_demo: bool) -> Self {
        let (settings, settings_error) = match GuiSettings::load() {
            Ok(settings) => (settings, None),
            Err(error) => (GuiSettings::default(), Some(error)),
        };
        Self::new_with_settings(window, cx, settings, settings_error, open_demo)
    }

    fn new_with_settings(
        window: &mut Window,
        cx: &mut Context<Self>,
        settings: GuiSettings,
        settings_error: Option<String>,
        open_demo: bool,
    ) -> Self {
        let backend_kind = settings
            .last_backend
            .map(BackendKind::from_recent_backend)
            .unwrap_or(BackendKind::Memory);
        let git_remote = input(window, cx, "Remote");
        let git_branch = input(window, cx, "Branch");
        let mongo_conn = input(window, cx, "Mongo connection string");
        let mongo_db = input(window, cx, "Database name");
        let proxy_url = input(window, cx, "Proxy URL");
        let proxy_token = input(window, cx, "Optional auth token");
        let input_subscriptions = vec![
            Self::refresh_on_input_change(&git_remote, cx),
            Self::refresh_on_input_change(&git_branch, cx),
            Self::refresh_on_input_change(&mongo_conn, cx),
            Self::refresh_on_input_change(&mongo_db, cx),
            Self::refresh_on_input_change(&proxy_url, cx),
            Self::refresh_on_input_change(&proxy_token, cx),
        ];

        let mut this = Self {
            backend_kind,
            settings,
            workspace: None,
            note_editors: RefCell::new(HashMap::new()),
            tree_state: cx.new(|cx| TreeState::new(cx)),
            loading: false,
            error: settings_error,
            file_path: None,
            redb_path: None,
            git_path: None,
            git_remote,
            git_branch,
            mongo_conn,
            mongo_db,
            proxy_url,
            proxy_token,
            _input_subscriptions: input_subscriptions,
        };

        this.restore_first_recent_for_selected_backend(window, cx);

        if open_demo {
            match Runtime::new()
                .map_err(|err| err.to_string())
                .and_then(|runtime| {
                    runtime
                        .block_on(WorkspaceModel::demo())
                        .map_err(|err| err.to_string())
                }) {
                Ok(workspace) => {
                    this.workspace = Some(workspace);
                    this.sync_tree_state(cx);
                }
                Err(error) => this.error = Some(error),
            }
        }

        this
    }

    fn refresh_on_input_change(input: &Entity<InputState>, cx: &mut Context<Self>) -> Subscription {
        cx.subscribe(input, |this, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                this.error = None;
                cx.notify();
            }
        })
    }

    fn select_backend(&mut self, kind: BackendKind, window: &mut Window, cx: &mut Context<Self>) {
        self.backend_kind = kind;
        self.restore_first_recent_for_selected_backend(window, cx);
        self.error = None;
        cx.notify();
    }

    fn open_workspace(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let config = match self.backend_config(cx) {
            Ok(config) => config,
            Err(error) => {
                self.error = Some(error);
                cx.notify();
                return;
            }
        };
        let recent = Self::recent_from_config(&config);
        let last_backend = self.backend_kind.to_recent_backend();

        self.loading = true;
        self.error = None;
        cx.notify();

        let result = Runtime::new()
            .map_err(|err| err.to_string())
            .and_then(|runtime| {
                runtime
                    .block_on(WorkspaceModel::open(config))
                    .map_err(|err| err.to_string())
            });

        match result {
            Ok(workspace) => {
                self.workspace = Some(workspace);
                self.note_editors.borrow_mut().clear();
                self.loading = false;
                if let Err(error) = self.persist_successful_open(last_backend, recent) {
                    self.error = Some(error);
                }
                self.sync_tree_state(cx);
            }
            Err(error) => {
                self.loading = false;
                self.error = Some(error);
            }
        }
        cx.notify();
    }

    fn recent_from_config(config: &BackendConfig) -> Option<RecentWorkspace> {
        match config {
            BackendConfig::Memory => None,
            BackendConfig::File { path } => Some(RecentWorkspace::File { path: path.clone() }),
            BackendConfig::Redb { path } => Some(RecentWorkspace::Redb { path: path.clone() }),
            BackendConfig::Git {
                path,
                remote,
                branch,
            } => Some(RecentWorkspace::Git {
                path: path.clone(),
                remote: remote.clone(),
                branch: branch.clone(),
            }),
            BackendConfig::Mongo { conn_str, db_name } => Some(RecentWorkspace::Mongo {
                conn_str: conn_str.clone(),
                db_name: db_name.clone(),
            }),
            BackendConfig::Proxy { url, .. } => Some(RecentWorkspace::Proxy { url: url.clone() }),
        }
    }

    fn persist_successful_open(
        &mut self,
        last_backend: RecentBackend,
        recent: Option<RecentWorkspace>,
    ) -> Result<(), String> {
        self.settings.last_backend = Some(last_backend);
        if let Some(recent) = recent {
            self.settings.upsert_recent(recent);
        }
        self.settings.save()
    }

    fn restore_first_recent_for_selected_backend(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(recent) = self
            .settings
            .first_recent_for_backend(self.backend_kind.to_recent_backend())
            .cloned()
        {
            self.apply_recent_workspace(&recent, window, cx);
        }
    }

    fn open_recent_workspace(
        &mut self,
        recent: &RecentWorkspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.loading {
            return;
        }

        self.apply_recent_workspace(recent, window, cx);
        self.error = None;
        self.open_workspace(window, cx);
    }

    fn apply_recent_workspace(
        &mut self,
        recent: &RecentWorkspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.backend_kind = BackendKind::from_recent_backend(recent.backend());
        match recent {
            RecentWorkspace::File { path } => {
                self.file_path = Some(PathBuf::from(path));
            }
            RecentWorkspace::Redb { path } => {
                self.redb_path = Some(PathBuf::from(path));
            }
            RecentWorkspace::Git {
                path,
                remote,
                branch,
            } => {
                self.git_path = Some(PathBuf::from(path));
                Self::set_input_value(&self.git_remote, remote, window, cx);
                Self::set_input_value(&self.git_branch, branch, window, cx);
            }
            RecentWorkspace::Mongo { conn_str, db_name } => {
                Self::set_input_value(&self.mongo_conn, conn_str, window, cx);
                Self::set_input_value(&self.mongo_db, db_name, window, cx);
            }
            RecentWorkspace::Proxy { url } => {
                Self::set_input_value(&self.proxy_url, url, window, cx);
                Self::set_input_value(&self.proxy_token, "", window, cx);
            }
        }
    }

    fn set_input_value(
        input: &Entity<InputState>,
        value: impl Into<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let value = value.into();
        input.update(cx, |input, cx| {
            input.set_value(value, window, cx);
        });
    }

    fn choose_path(&mut self, target: PathTarget, cx: &mut Context<Self>) {
        self.error = None;
        let receiver = cx.prompt_for_paths(target.options());

        cx.spawn(async move |view, cx| match receiver.await {
            Ok(Ok(Some(paths))) => {
                if let Some(path) = paths.into_iter().next() {
                    let _ = view.update(cx, |this, cx| {
                        this.set_selected_path(target, path);
                        cx.notify();
                    });
                }
            }
            Ok(Ok(None)) => {}
            Ok(Err(error)) => {
                let _ = view.update(cx, |this, cx| {
                    this.error = Some(error.to_string());
                    cx.notify();
                });
            }
            Err(error) => {
                let _ = view.update(cx, |this, cx| {
                    this.error = Some(error.to_string());
                    cx.notify();
                });
            }
        })
        .detach();

        cx.notify();
    }

    fn set_selected_path(&mut self, target: PathTarget, path: PathBuf) {
        match target {
            PathTarget::FileDirectory => self.file_path = Some(path),
            PathTarget::RedbFile => self.redb_path = Some(path),
            PathTarget::GitDirectory => self.git_path = Some(path),
        }
    }

    fn selected_path(&self, target: PathTarget) -> Option<&PathBuf> {
        match target {
            PathTarget::FileDirectory => self.file_path.as_ref(),
            PathTarget::RedbFile => self.redb_path.as_ref(),
            PathTarget::GitDirectory => self.git_path.as_ref(),
        }
    }

    fn open_demo(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.loading = true;
        self.error = None;
        cx.notify();

        let result = Runtime::new()
            .map_err(|err| err.to_string())
            .and_then(|runtime| {
                runtime
                    .block_on(WorkspaceModel::demo())
                    .map_err(|err| err.to_string())
            });

        match result {
            Ok(workspace) => {
                self.workspace = Some(workspace);
                self.note_editors.borrow_mut().clear();
                self.loading = false;
                self.sync_tree_state(cx);
            }
            Err(error) => {
                self.loading = false;
                self.error = Some(error);
            }
        }
        cx.notify();
    }

    fn activate_tree_item(
        &mut self,
        item_id: String,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(directory_id) = item_id.strip_prefix(DIRECTORY_ITEM_PREFIX) {
            self.activate_item(TreeRowKind::Directory, directory_id.to_owned(), cx);
        } else if let Some(note_id) = item_id.strip_prefix(NOTE_ITEM_PREFIX) {
            self.activate_item(TreeRowKind::Note, note_id.to_owned(), cx);
        }
    }

    fn activate_item(&mut self, kind: TreeRowKind, id: String, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.as_mut() else {
            return;
        };

        let result = Runtime::new()
            .map_err(|err| err.to_string())
            .and_then(|runtime| match kind {
                TreeRowKind::Directory => runtime
                    .block_on(workspace.toggle_directory(id))
                    .map_err(|err| err.to_string()),
                TreeRowKind::Note => runtime
                    .block_on(workspace.open_note(id))
                    .map_err(|err| err.to_string()),
            });

        if let Err(error) = result {
            self.error = Some(error);
        } else {
            self.sync_tree_state(cx);
        }
        cx.notify();
    }

    fn select_note_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.as_mut() else {
            return;
        };

        if let Err(error) = workspace.select_tab(index) {
            self.error = Some(error.to_string());
        } else {
            self.sync_tree_state(cx);
        }
        cx.notify();
    }

    fn close_note_tab(&mut self, note_id: String, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.as_mut() else {
            return;
        };

        if let Err(error) = workspace.close_note_tab(note_id.clone()) {
            self.error = Some(error.to_string());
        } else {
            self.note_editors.borrow_mut().remove(&note_id);
            self.sync_tree_state(cx);
        }
        cx.notify();
    }

    fn move_note_tab_before(
        &mut self,
        note_id: String,
        target_index: usize,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.as_mut() else {
            return;
        };

        if let Err(error) = workspace.move_note_tab_before(note_id, target_index) {
            self.error = Some(error.to_string());
        }
        cx.notify();
    }

    fn move_note_tab_to_end(&mut self, note_id: String, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.as_mut() else {
            return;
        };

        if let Err(error) = workspace.move_note_tab_to_end(note_id) {
            self.error = Some(error.to_string());
        }
        cx.notify();
    }

    fn sync_tree_state(&mut self, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.as_ref() else {
            return;
        };

        let selected_index = workspace.tree_rows().iter().position(|row| row.selected);
        let items = vec![tree_item_for_node(&workspace.root)];

        self.tree_state.update(cx, |state, cx| {
            state.set_items(items, cx);
            state.set_selected_index(selected_index, cx);
        });
    }

    fn backend_config(&self, cx: &App) -> Result<BackendConfig, String> {
        match self.backend_kind {
            BackendKind::Memory => Ok(BackendConfig::Memory),
            BackendKind::File => Ok(BackendConfig::File {
                path: self
                    .required_path(PathTarget::FileDirectory, "Choose a file storage folder")?,
            }),
            BackendKind::Redb => Ok(BackendConfig::Redb {
                path: self.required_path(PathTarget::RedbFile, "Choose a redb file")?,
            }),
            BackendKind::Git => Ok(BackendConfig::Git {
                path: self
                    .required_path(PathTarget::GitDirectory, "Choose a git storage folder")?,
                remote: read_input(&self.git_remote, cx),
                branch: read_input(&self.git_branch, cx),
            }),
            BackendKind::Mongo => Ok(BackendConfig::Mongo {
                conn_str: required_input(&self.mongo_conn, cx, "Enter a Mongo connection string")?,
                db_name: required_input(&self.mongo_db, cx, "Enter a Mongo database name")?,
            }),
            BackendKind::Proxy => Ok(BackendConfig::Proxy {
                url: required_input(&self.proxy_url, cx, "Enter a proxy URL")?,
                auth_token: optional_input(&self.proxy_token, cx),
            }),
        }
    }

    fn can_open_workspace(&self, cx: &App) -> bool {
        self.backend_config(cx).is_ok()
    }

    fn required_path(&self, target: PathTarget, message: &'static str) -> Result<String, String> {
        self.selected_path(target)
            .map(|path| path.to_string_lossy().into_owned())
            .filter(|path| !path.trim().is_empty())
            .ok_or_else(|| message.to_owned())
    }

    fn render_open_screen(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(cx.theme().background)
            .overflow_y_scrollbar()
            .child(
                v_flex()
                    .w_full()
                    .max_w(px(720.))
                    .p_6()
                    .gap_5()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                h_flex()
                                    .items_center()
                                    .flex_wrap()
                                    .gap_2()
                                    .child(Label::new("Glues").text_2xl().font_semibold())
                                    .child(Tag::secondary().outline().small().child("READ ONLY")),
                            )
                            .child(
                                Label::new("Open a workspace")
                                    .secondary("Browse notes read-only from an existing backend."),
                            ),
                    )
                    .child(
                        GroupBox::new()
                            .outline()
                            .title(Label::new("Backend"))
                            .w_full()
                            .child(
                                v_flex()
                                    .gap_4()
                                    .child(self.render_backend_selector(cx))
                                    .child(self.render_backend_fields(cx))
                                    .child(self.render_open_actions(cx)),
                            ),
                    )
                    .children(self.error.as_ref().map(|error| {
                        Alert::error("open-workspace-error", error.clone())
                            .w_full()
                            .into_any_element()
                    }))
                    .when(!self.settings.recent_workspaces.is_empty(), |this| {
                        this.child(self.render_recent_workspaces(cx))
                    }),
            )
    }

    fn render_backend_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        ButtonGroup::new("backend-selector")
            .w_full()
            .flex_wrap()
            .compact()
            .outline()
            .children(BackendKind::ALL.into_iter().map(|kind| {
                Button::new(kind.key())
                    .label(kind.label())
                    .selected(self.backend_kind == kind)
            }))
            .on_click(cx.listener(|this, clicks: &Vec<usize>, window, cx| {
                if let Some(kind) = clicks
                    .last()
                    .and_then(|index| BackendKind::ALL.get(*index))
                    .copied()
                {
                    this.select_backend(kind, window, cx);
                }
            }))
    }

    fn render_backend_fields(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Form::vertical()
            .when(self.backend_kind == BackendKind::Memory, |this| {
                this.child(
                    Field::new()
                        .label("Storage")
                        .description("No path required.")
                        .child(Label::new("Memory").secondary("empty in-memory workspace")),
                )
            })
            .when(self.backend_kind == BackendKind::File, |this| {
                this.child(self.path_field("File storage", PathTarget::FileDirectory, cx))
            })
            .when(self.backend_kind == BackendKind::Redb, |this| {
                this.child(self.path_field("redb file", PathTarget::RedbFile, cx))
            })
            .when(self.backend_kind == BackendKind::Git, |this| {
                this.child(self.path_field("Git storage", PathTarget::GitDirectory, cx))
                    .child(input_field("Remote", &self.git_remote))
                    .child(input_field("Branch", &self.git_branch))
            })
            .when(self.backend_kind == BackendKind::Mongo, |this| {
                this.child(input_field("Connection string", &self.mongo_conn))
                    .child(input_field("Database", &self.mongo_db))
            })
            .when(self.backend_kind == BackendKind::Proxy, |this| {
                this.child(input_field("Proxy URL", &self.proxy_url))
                    .child(input_field("Auth token", &self.proxy_token))
            })
    }

    fn render_open_actions(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let can_open = self.can_open_workspace(cx);

        h_flex()
            .gap_2()
            .items_start()
            .flex_wrap()
            .child(
                Button::new("open-workspace")
                    .primary()
                    .label("Open workspace")
                    .loading(self.loading)
                    .disabled(!can_open || self.loading)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.open_workspace(window, cx);
                    })),
            )
            .child(
                Button::new("open-demo")
                    .ghost()
                    .label("Open demo")
                    .disabled(self.loading)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.open_demo(window, cx);
                    })),
            )
    }

    fn render_recent_workspaces(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let rows = self
            .settings
            .recent_workspaces
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, recent)| {
                self.render_recent_workspace_row(index, recent, cx)
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        GroupBox::new()
            .outline()
            .title(Label::new("Recent workspaces"))
            .w_full()
            .child(v_flex().gap_1().children(rows))
    }

    fn render_recent_workspace_row(
        &self,
        index: usize,
        recent: RecentWorkspace,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let icon = recent_icon(&recent);
        let title = recent.title();
        let subtitle = recent.subtitle();
        let backend_label = recent.backend_label();
        let click_recent = recent.clone();

        ListItem::new(("recent-workspace", index))
            .child(
                h_flex()
                    .w_full()
                    .min_w_0()
                    .gap_2()
                    .items_center()
                    .child(Icon::new(icon).small())
                    .child(Tag::secondary().outline().small().child(backend_label))
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w_0()
                            .gap_1()
                            .child(Label::new(title).truncate())
                            .child(
                                Label::new(subtitle)
                                    .text_sm()
                                    .truncate()
                                    .text_color(cx.theme().muted_foreground),
                            ),
                    )
                    .child(Icon::new(IconName::ChevronRight).small()),
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.open_recent_workspace(&click_recent, window, cx);
            }))
    }

    fn path_field(&self, label: &'static str, target: PathTarget, cx: &mut Context<Self>) -> Field {
        let selected = self
            .selected_path(target)
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| target.empty_label().to_owned());
        let has_selection = self.selected_path(target).is_some();
        let description = if has_selection {
            "Path selected"
        } else {
            target.empty_description()
        };

        Field::new().label(label).description(description).child(
            h_flex()
                .gap_2()
                .items_center()
                .child(
                    Button::new(target.button_id())
                        .icon(target.icon())
                        .label(if has_selection {
                            target.change_label()
                        } else {
                            target.choose_label()
                        })
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.choose_path(target, cx);
                        })),
                )
                .child(
                    Label::new(selected)
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .when(!has_selection, |label| {
                            label.text_color(cx.theme().muted_foreground)
                        }),
                ),
        )
    }

    fn render_workspace(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let workspace = self
            .workspace
            .as_ref()
            .expect("workspace should exist before rendering workspace");

        h_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(
                v_flex()
                    .w(px(340.))
                    .h_full()
                    .p_3()
                    .gap_3()
                    .border_r_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().sidebar)
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .px_2()
                            .child(Icon::new(IconName::BookOpen).small())
                            .child(Label::new("Notebook").font_semibold()),
                    )
                    .child(self.render_tree(cx)),
            )
            .child(
                v_flex()
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .min_h(px(0.))
                    .child(self.render_note_viewer(workspace, window, cx)),
            )
            .children(self.error.as_ref().map(|error| {
                div()
                    .absolute()
                    .right_4()
                    .bottom_4()
                    .child(Alert::error("workspace-error", error.clone()))
            }))
    }

    fn render_tree(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();

        tree(
            &self.tree_state,
            move |index, entry, selected, _window, _cx| {
                let item = entry.item();
                let item_id = item.id.to_string();
                let icon = if item_id.starts_with(DIRECTORY_ITEM_PREFIX) {
                    if entry.is_expanded() {
                        IconName::FolderOpen
                    } else {
                        IconName::Folder
                    }
                } else {
                    IconName::File
                };

                ListItem::new(("tree-item", index))
                    .selected(selected)
                    .pl(px(12.) + px(18.) * entry.depth())
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(Icon::new(icon).small())
                            .child(Label::new(item.label.clone())),
                    )
                    .on_click({
                        let view = view.clone();
                        move |_, window, cx| {
                            let item_id = item_id.clone();
                            view.update(cx, |this, cx| {
                                this.activate_tree_item(item_id, window, cx);
                            });
                        }
                    })
            },
        )
        .size_full()
    }

    fn render_note_viewer(
        &self,
        workspace: &WorkspaceModel,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if let Some(opened) = workspace.active_note() {
            v_flex()
                .flex_1()
                .h_full()
                .min_h(px(0.))
                .child(self.render_note_tabs(workspace, cx))
                .child(
                    v_flex()
                        .flex_1()
                        .min_h(px(0.))
                        .pt_3()
                        .gap_2()
                        .child(
                            div()
                                .px_6()
                                .text_sm()
                                .text_color(cx.theme().muted_foreground)
                                .child(note_breadcrumb(workspace)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .min_h(px(0.))
                                .overflow_hidden()
                                .border_t_1()
                                .border_color(cx.theme().border)
                                .child(
                                    Input::new(&self.note_editor(opened, window, cx))
                                        .appearance(false)
                                        .disabled(true)
                                        .text_base()
                                        .h_full(),
                                ),
                        ),
                )
                .into_any_element()
        } else {
            v_flex()
                .flex_1()
                .h_full()
                .min_h(px(0.))
                .p_6()
                .child(
                    v_flex()
                        .flex_1()
                        .gap_2()
                        .items_start()
                        .child(Label::new("No note open").font_semibold())
                        .child(
                            Label::new("Select a note from the tree to preview its content.")
                                .text_color(cx.theme().muted_foreground),
                        ),
                )
                .into_any_element()
        }
    }

    fn render_note_tabs(
        &self,
        workspace: &WorkspaceModel,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let view = cx.entity();
        let end_drop_view = view.clone();
        let mut tab_bar = TabBar::new("note-tabs")
            .w_full()
            .small()
            .bg(cx.theme().tab_bar)
            .last_empty_space(
                div()
                    .id("note-tabs-empty-space")
                    .h_full()
                    .flex_grow()
                    .min_w_16()
                    .drag_over::<DraggedNoteTab>(|this, _, _, cx| this.bg(cx.theme().drop_target))
                    .on_drop(move |drag: &DraggedNoteTab, _, cx| {
                        let note_id = drag.note_id.clone();
                        end_drop_view.update(cx, |this, cx| {
                            this.move_note_tab_to_end(note_id, cx);
                        });
                    }),
            )
            .suffix(div().w_0().h_full());

        let active_index = workspace.active_tab_index();
        if let Some(active_index) = active_index {
            tab_bar = tab_bar.selected_index(active_index);
        }

        for (index, opened) in workspace.open_tabs().iter().enumerate() {
            let select_view = view.clone();
            let close_view = view.clone();
            let drop_view = view.clone();
            let note_id = opened.note.id.clone();
            let close_note_id = note_id.clone();
            let drop_note_id = note_id.clone();
            let close_group_id = format!("note-tab-close-{note_id}");
            let drag_info = DraggedNoteTab {
                note_id: note_id.clone(),
                label: opened.note.name.clone(),
            };

            tab_bar = tab_bar.child(
                Tab::new()
                    .group(close_group_id.clone())
                    .cursor_grab()
                    .max_w(px(240.))
                    .relative()
                    .child(
                        h_flex()
                            .px(px(6.))
                            .items_center()
                            .justify_center()
                            .overflow_hidden()
                            .child(Label::new(opened.note.name.clone()).truncate()),
                    )
                    .child(
                        h_flex()
                            .id(format!("close-note-tab-{note_id}"))
                            .absolute()
                            .right(px(2.))
                            .top(px(3.))
                            .invisible()
                            .w(px(16.))
                            .h(px(16.))
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .rounded(px(3.))
                            .group_hover(&close_group_id, |this| this.visible())
                            .hover(|this| this.bg(cx.theme().secondary_hover))
                            .active(|this| this.bg(cx.theme().secondary_active))
                            .child(
                                Icon::new(IconName::Close)
                                    .with_size(px(10.))
                                    .text_color(cx.theme().muted_foreground),
                            )
                            .on_click(move |_, _, cx| {
                                cx.stop_propagation();
                                let note_id = close_note_id.clone();
                                close_view.update(cx, |this, cx| {
                                    this.close_note_tab(note_id, cx);
                                });
                            }),
                    )
                    .on_drag(drag_info, |drag, _, _, cx| cx.new(|_| drag.clone()))
                    .drag_over::<DraggedNoteTab>(move |this, drag, _, cx| {
                        if drag.note_id == drop_note_id {
                            this
                        } else {
                            this.border_l_2().border_color(cx.theme().drag_border)
                        }
                    })
                    .on_drop(move |drag: &DraggedNoteTab, _, cx| {
                        if drag.note_id == note_id {
                            return;
                        }

                        let note_id = drag.note_id.clone();
                        drop_view.update(cx, |this, cx| {
                            this.move_note_tab_before(note_id, index, cx);
                        });
                    })
                    .on_click(move |_, _, cx| {
                        select_view.update(cx, |this, cx| {
                            this.select_note_tab(index, cx);
                        });
                    }),
            );
        }

        div()
            .relative()
            .w_full()
            .bg(cx.theme().tab_bar)
            .child(tab_bar)
            .child(
                div()
                    .id("note-tabs-left-cover")
                    .absolute()
                    .left_0()
                    .top_0()
                    .bottom_0()
                    .w(px(1.))
                    .bg(cx.theme().tab_bar),
            )
    }

    fn note_editor(
        &self,
        opened: &OpenedNote,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<InputState> {
        let note_id = opened.note.id.clone();
        if let Some(editor) = self.note_editors.borrow().get(&note_id).cloned() {
            return editor;
        }

        let content = opened.content.clone();
        let editor = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("markdown")
                .line_number(true)
                .folding(false)
                .searchable(true)
                .soft_wrap(true)
                .default_value(content.clone())
        });
        self.note_editors
            .borrow_mut()
            .insert(note_id, editor.clone());
        editor
    }
}

impl Render for GluesGui {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.workspace.is_some() {
            self.render_workspace(window, cx).into_any_element()
        } else {
            self.render_open_screen(cx).into_any_element()
        }
    }
}

fn tree_item_for_node(node: &TreeNode) -> TreeItem {
    let mut item = TreeItem::new(
        directory_item_id(&node.directory.id),
        node.directory.name.clone(),
    )
    .expanded(node.expanded);

    if !node.loaded {
        item = item.child(
            TreeItem::new(
                format!("{LOADING_ITEM_PREFIX}{}", node.directory.id),
                "Loading...",
            )
            .disabled(true),
        );
    }

    for directory in &node.directories {
        item = item.child(tree_item_for_node(directory));
    }

    for note in &node.notes {
        item = item.child(TreeItem::new(note_item_id(&note.id), note.name.clone()));
    }

    item
}

fn directory_item_id(id: &str) -> String {
    format!("{DIRECTORY_ITEM_PREFIX}{id}")
}

fn note_item_id(id: &str) -> String {
    format!("{NOTE_ITEM_PREFIX}{id}")
}

fn input_field(label: &'static str, input: &Entity<InputState>) -> Field {
    Field::new().label(label).child(Input::new(input))
}

fn recent_icon(recent: &RecentWorkspace) -> IconName {
    match recent {
        RecentWorkspace::File { .. } => IconName::FolderOpen,
        RecentWorkspace::Redb { .. } => IconName::File,
        RecentWorkspace::Git { .. } => IconName::Github,
        RecentWorkspace::Mongo { .. } => IconName::Building2,
        RecentWorkspace::Proxy { .. } => IconName::Globe,
    }
}

fn note_breadcrumb(workspace: &WorkspaceModel) -> Breadcrumb {
    let mut path = workspace.opened_note_directory_path();
    if let Some(opened) = workspace.active_note() {
        path.push(opened.note.name.clone());
    }

    Breadcrumb::new().children(path)
}

fn input(
    window: &mut Window,
    cx: &mut Context<GluesGui>,
    placeholder: &'static str,
) -> Entity<InputState> {
    cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
}

fn read_input(input: &Entity<InputState>, cx: &App) -> String {
    input.read(cx).value().to_string()
}

fn required_input(
    input: &Entity<InputState>,
    cx: &App,
    message: &'static str,
) -> Result<String, String> {
    let value = read_input(input, cx);
    if value.trim().is_empty() {
        Err(message.to_owned())
    } else {
        Ok(value)
    }
}

fn optional_input(input: &Entity<InputState>, cx: &App) -> Option<String> {
    let value = read_input(input, cx);
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}
