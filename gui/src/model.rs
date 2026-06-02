use glues_core::{
    Error,
    backend::{BackendBox, CoreBackend, local::Db, proxy::ProxyClient},
    data::{Directory, Note},
    types::{DirectoryId, NoteId},
};

pub type Result<T> = std::result::Result<T, Error>;

pub struct WorkspaceModel {
    backend: BackendBox,
    pub root: TreeNode,
    pub selected: TreeSelection,
    open_tabs: Vec<OpenedNote>,
    active_tab_index: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub directory: Directory,
    pub directories: Vec<TreeNode>,
    pub notes: Vec<Note>,
    pub expanded: bool,
    pub loaded: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeSelection {
    Directory(DirectoryId),
    Note(NoteId),
}

#[derive(Clone, Debug)]
pub struct OpenedNote {
    pub note: Note,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeRowKind {
    Directory,
    Note,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeRow {
    pub id: String,
    pub depth: usize,
    pub kind: TreeRowKind,
    pub name: String,
    pub expanded: bool,
    pub selected: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkspaceBackendConfig {
    Memory,
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
        auth_token: Option<String>,
    },
}

impl WorkspaceModel {
    pub async fn open(config: WorkspaceBackendConfig) -> Result<Self> {
        let backend = Self::open_backend(config).await?;
        Self::from_backend(backend).await
    }

    pub async fn demo() -> Result<Self> {
        Self::demo_with_open_tabs(false).await
    }

    #[cfg(feature = "visual-tests")]
    pub async fn visual_demo() -> Result<Self> {
        Self::demo_with_open_tabs(true).await
    }

    #[cfg(feature = "visual-tests")]
    pub async fn visual_empty_viewer() -> Result<Self> {
        let mut backend = Self::open_backend(WorkspaceBackendConfig::Memory).await?;
        let root_id = backend.root_id();
        backend
            .add_directory(root_id.clone(), "Daily notes".to_owned())
            .await?;
        backend
            .add_directory(root_id.clone(), "Projects".to_owned())
            .await?;
        backend
            .add_note(root_id.clone(), "README".to_owned())
            .await?;

        Self::from_backend(backend).await
    }

    #[cfg(feature = "visual-tests")]
    pub async fn visual_long_note() -> Result<Self> {
        let mut backend = Self::open_backend(WorkspaceBackendConfig::Memory).await?;
        let root_id = backend.root_id();
        let notes = backend
            .add_directory(root_id.clone(), "Notes".to_owned())
            .await?;
        let notes_id = notes.id.clone();
        let note = backend
            .add_note(notes_id.clone(), "Long wrapping note".to_owned())
            .await?;
        let note_id = note.id.clone();
        backend
            .update_note_content(note.id, visual_long_note_content())
            .await?;

        let mut workspace = Self::from_backend(backend).await?;
        workspace.toggle_directory(notes_id).await?;
        workspace.open_note(note_id).await?;
        Ok(workspace)
    }

    async fn demo_with_open_tabs(open_multiple_tabs: bool) -> Result<Self> {
        let mut backend = Self::open_backend(WorkspaceBackendConfig::Memory).await?;
        let root_id = backend.root_id();
        let gui = backend
            .add_directory(root_id.clone(), "Glues GUI".to_owned())
            .await?;
        let gui_id = gui.id.clone();
        let testing = backend
            .add_directory(root_id.clone(), "Testing".to_owned())
            .await?;
        let testing_id = testing.id.clone();

        let overview = backend
            .add_note(root_id.clone(), "GPUI migration overview".to_owned())
            .await?;
        let overview_id = overview.id.clone();
        backend
            .update_note_content(
                overview.id.clone(),
                "# GPUI migration overview\n\nCurrent milestone: read-only GUI over real backends.\n\nBackend selection happens inside the GUI.\n\nThe old core state machine is not used by this frontend."
                    .to_owned(),
            )
            .await?;

        let backend_note = backend
            .add_note(gui.id.clone(), "Backend opening".to_owned())
            .await?;
        let backend_note_id = backend_note.id.clone();
        backend
            .update_note_content(
                backend_note.id,
                "Keep GUI backend opening local to the frontend while using core backend implementations."
                    .to_owned(),
            )
            .await?;

        let visual = backend
            .add_note(testing.id, "Visual review".to_owned())
            .await?;
        let visual_id = visual.id.clone();
        backend
            .update_note_content(
                visual.id,
                "Local macOS screenshots are review artifacts, not committed baselines.".to_owned(),
            )
            .await?;

        let mut workspace = Self::from_backend(backend).await?;
        workspace.open_note(overview_id).await?;
        if open_multiple_tabs {
            workspace.toggle_directory(gui_id).await?;
            workspace.toggle_directory(testing_id).await?;
            workspace.open_note(backend_note_id).await?;
            workspace.open_note(visual_id).await?;
            workspace.select_tab(0)?;
        }
        Ok(workspace)
    }

    async fn open_backend(config: WorkspaceBackendConfig) -> Result<BackendBox> {
        let backend: BackendBox = match config {
            WorkspaceBackendConfig::Memory => Box::new(Db::memory().await?),
            WorkspaceBackendConfig::File { path } => Box::new(Db::file(&path).await?),
            WorkspaceBackendConfig::Redb { path } => Box::new(Db::redb(&path).await?),
            WorkspaceBackendConfig::Git {
                path,
                remote,
                branch,
            } => Box::new(Db::git(&path, remote, branch).await?),
            WorkspaceBackendConfig::Mongo { conn_str, db_name } => {
                Box::new(Db::mongo(&conn_str, &db_name).await?)
            }
            WorkspaceBackendConfig::Proxy { url, auth_token } => {
                Box::new(ProxyClient::connect(url, auth_token).await?)
            }
        };

        Ok(backend)
    }

    async fn from_backend(mut backend: BackendBox) -> Result<Self> {
        let root_id = backend.root_id();
        let root_directory = backend.fetch_directory(root_id.clone()).await?;
        let root = Self::load_node(&mut backend, root_directory, true).await?;
        let selected = TreeSelection::Directory(root_id);

        Ok(Self {
            backend,
            root,
            selected,
            open_tabs: Vec::new(),
            active_tab_index: None,
        })
    }

    async fn load_node(
        backend: &mut BackendBox,
        directory: Directory,
        expanded: bool,
    ) -> Result<TreeNode> {
        let notes = backend.fetch_notes(directory.id.clone()).await?;
        let directories = backend
            .fetch_directories(directory.id.clone())
            .await?
            .into_iter()
            .map(TreeNode::unloaded)
            .collect();

        Ok(TreeNode {
            directory,
            directories,
            notes,
            expanded,
            loaded: true,
        })
    }

    pub async fn toggle_directory(&mut self, directory_id: DirectoryId) -> Result<()> {
        let loaded = self
            .root
            .find_directory(&directory_id)
            .map(|node| node.loaded)
            .ok_or_else(|| Error::NotFound(format!("directory not found: {directory_id}")))?;

        if loaded {
            let node = self
                .root
                .find_directory_mut(&directory_id)
                .ok_or_else(|| Error::NotFound(format!("directory not found: {directory_id}")))?;
            node.expanded = !node.expanded;
        } else {
            let notes = self.backend.fetch_notes(directory_id.clone()).await?;
            let directories = self
                .backend
                .fetch_directories(directory_id.clone())
                .await?
                .into_iter()
                .map(TreeNode::unloaded)
                .collect();
            let node = self
                .root
                .find_directory_mut(&directory_id)
                .ok_or_else(|| Error::NotFound(format!("directory not found: {directory_id}")))?;
            node.notes = notes;
            node.directories = directories;
            node.loaded = true;
            node.expanded = true;
        }

        self.selected = TreeSelection::Directory(directory_id);
        Ok(())
    }

    pub async fn open_note(&mut self, note_id: NoteId) -> Result<()> {
        if let Some(index) = self.open_tabs.iter().position(|tab| tab.note.id == note_id) {
            self.active_tab_index = Some(index);
            self.selected = TreeSelection::Note(note_id);
            return Ok(());
        }

        let note = self
            .root
            .find_note(&note_id)
            .cloned()
            .ok_or_else(|| Error::NotFound(format!("note not found: {note_id}")))?;
        let content = self.backend.fetch_note_content(note_id.clone()).await?;
        self.selected = TreeSelection::Note(note_id);
        self.open_tabs.push(OpenedNote { note, content });
        self.active_tab_index = Some(self.open_tabs.len() - 1);
        Ok(())
    }

    pub fn select_tab(&mut self, index: usize) -> Result<()> {
        let tab = self
            .open_tabs
            .get(index)
            .ok_or_else(|| Error::NotFound(format!("tab not found: {index}")))?;

        self.active_tab_index = Some(index);
        self.selected = TreeSelection::Note(tab.note.id.clone());
        Ok(())
    }

    pub fn close_note_tab(&mut self, note_id: NoteId) -> Result<()> {
        let index = self
            .open_tabs
            .iter()
            .position(|tab| tab.note.id == note_id)
            .ok_or_else(|| Error::NotFound(format!("tab not found: {note_id}")))?;
        let active_index = self.active_tab_index;

        self.open_tabs.remove(index);

        match active_index {
            None => {}
            Some(_) if self.open_tabs.is_empty() => {
                self.active_tab_index = None;
            }
            Some(active_index) if active_index == index => {
                let next_index = index.min(self.open_tabs.len() - 1);
                self.active_tab_index = Some(next_index);
                self.selected = TreeSelection::Note(self.open_tabs[next_index].note.id.clone());
            }
            Some(active_index) if index < active_index => {
                let next_index = active_index - 1;
                self.active_tab_index = Some(next_index);
                self.selected = TreeSelection::Note(self.open_tabs[next_index].note.id.clone());
            }
            Some(active_index) => {
                self.active_tab_index = Some(active_index);
            }
        }

        Ok(())
    }

    pub fn move_note_tab_before(&mut self, note_id: NoteId, target_index: usize) -> Result<()> {
        if target_index >= self.open_tabs.len() {
            return Err(Error::NotFound(format!("tab not found: {target_index}")));
        }

        let index = self
            .open_tabs
            .iter()
            .position(|tab| tab.note.id == note_id)
            .ok_or_else(|| Error::NotFound(format!("tab not found: {note_id}")))?;

        if index == target_index {
            return Ok(());
        }

        let active_note_id = self.active_note().map(|opened| opened.note.id.clone());
        let tab = self.open_tabs.remove(index);
        let target_index = if index < target_index {
            target_index - 1
        } else {
            target_index
        };
        self.open_tabs.insert(target_index, tab);
        self.restore_active_tab(active_note_id);
        Ok(())
    }

    pub fn move_note_tab_to_end(&mut self, note_id: NoteId) -> Result<()> {
        let index = self
            .open_tabs
            .iter()
            .position(|tab| tab.note.id == note_id)
            .ok_or_else(|| Error::NotFound(format!("tab not found: {note_id}")))?;

        if index + 1 == self.open_tabs.len() {
            return Ok(());
        }

        let active_note_id = self.active_note().map(|opened| opened.note.id.clone());
        let tab = self.open_tabs.remove(index);
        self.open_tabs.push(tab);
        self.restore_active_tab(active_note_id);
        Ok(())
    }

    fn restore_active_tab(&mut self, active_note_id: Option<NoteId>) {
        let Some(active_note_id) = active_note_id else {
            self.active_tab_index = None;
            return;
        };

        self.active_tab_index = self
            .open_tabs
            .iter()
            .position(|tab| tab.note.id == active_note_id);
    }

    pub fn active_note(&self) -> Option<&OpenedNote> {
        self.active_tab_index
            .and_then(|index| self.open_tabs.get(index))
    }

    pub fn active_tab_index(&self) -> Option<usize> {
        self.active_tab_index
    }

    pub fn open_tabs(&self) -> &[OpenedNote] {
        &self.open_tabs
    }

    pub fn tree_rows(&self) -> Vec<TreeRow> {
        let mut rows = Vec::new();
        self.root.append_rows(0, &self.selected, &mut rows);
        rows
    }

    pub fn opened_note_directory_path(&self) -> Vec<String> {
        let Some(opened) = self.active_note() else {
            return Vec::new();
        };

        self.root
            .directory_path(&opened.note.directory_id)
            .unwrap_or_default()
    }

    pub fn snapshot_workspace(&self) -> String {
        let mut lines = Vec::new();
        lines.push(match &self.selected {
            TreeSelection::Directory(id) => {
                let name = self
                    .root
                    .find_directory(id)
                    .map(|node| node.directory.name.as_str())
                    .unwrap_or("<missing>");
                format!("selected: directory:{name}")
            }
            TreeSelection::Note(id) => {
                let name = self
                    .root
                    .find_note(id)
                    .map(|note| note.name.as_str())
                    .unwrap_or("<missing>");
                format!("selected: note:{name}")
            }
        });
        lines.push("tree:".to_owned());
        for row in self.tree_rows() {
            let indent = "  ".repeat(row.depth);
            let marker = match row.kind {
                TreeRowKind::Directory if row.expanded => "v",
                TreeRowKind::Directory => ">",
                TreeRowKind::Note => "-",
            };
            let selected = if row.selected { "*" } else { " " };
            lines.push(format!("{indent}{selected} {marker} {}", row.name));
        }
        lines.push("tabs:".to_owned());
        if self.open_tabs.is_empty() {
            lines.push("  <empty>".to_owned());
        } else {
            for (index, tab) in self.open_tabs.iter().enumerate() {
                let active = if Some(index) == self.active_tab_index {
                    "*"
                } else {
                    " "
                };
                lines.push(format!("  {active} {}", tab.note.name));
            }
        }
        lines.push("viewer:".to_owned());
        if let Some(opened) = self.active_note() {
            lines.push(format!("  note: {}", opened.note.name));
            lines.push("  content:".to_owned());
            for line in opened.content.lines() {
                lines.push(format!("    {line}"));
            }
        } else {
            lines.push("  <empty>".to_owned());
        }
        lines.join("\n")
    }
}

#[cfg(feature = "visual-tests")]
fn visual_long_note_content() -> String {
    [
        "# Long wrapping note",
        "",
        "This viewer keeps the source text intact, but long lines should wrap inside the editor instead of forcing horizontal scrolling in the first read-only pass.",
        "",
        "The line-number gutter, breadcrumb, and tab bar should stay fixed while the note content uses the remaining vertical space.",
        "",
        "- Backend selection stays outside the tree.",
        "- Opening a note creates a tab.",
        "- Closing and reordering tabs should not disturb the visible editor layout.",
        "",
        "A second deliberately long sentence checks that wrapping continues to use the same left edge after the first visual line, making the read-only view feel like an editor instead of a formatted document preview.",
        "",
        "End of sample.",
    ]
    .join("\n")
}

impl TreeNode {
    fn unloaded(directory: Directory) -> Self {
        Self {
            directory,
            directories: Vec::new(),
            notes: Vec::new(),
            expanded: false,
            loaded: false,
        }
    }

    fn find_directory(&self, directory_id: &DirectoryId) -> Option<&Self> {
        if &self.directory.id == directory_id {
            return Some(self);
        }

        self.directories
            .iter()
            .find_map(|node| node.find_directory(directory_id))
    }

    fn find_directory_mut(&mut self, directory_id: &DirectoryId) -> Option<&mut Self> {
        if &self.directory.id == directory_id {
            return Some(self);
        }

        self.directories
            .iter_mut()
            .find_map(|node| node.find_directory_mut(directory_id))
    }

    fn find_note(&self, note_id: &NoteId) -> Option<&Note> {
        self.notes
            .iter()
            .find(|note| &note.id == note_id)
            .or_else(|| {
                self.directories
                    .iter()
                    .find_map(|node| node.find_note(note_id))
            })
    }

    #[cfg(test)]
    fn find_note_by_name(&self, name: &str) -> Option<&Note> {
        self.notes
            .iter()
            .find(|note| note.name == name)
            .or_else(|| {
                self.directories
                    .iter()
                    .find_map(|node| node.find_note_by_name(name))
            })
    }

    fn directory_path(&self, directory_id: &DirectoryId) -> Option<Vec<String>> {
        if &self.directory.id == directory_id {
            return Some(vec![self.directory.name.clone()]);
        }

        self.directories.iter().find_map(|node| {
            let mut path = node.directory_path(directory_id)?;
            path.insert(0, self.directory.name.clone());
            Some(path)
        })
    }

    fn append_rows(&self, depth: usize, selected: &TreeSelection, rows: &mut Vec<TreeRow>) {
        rows.push(TreeRow {
            id: self.directory.id.clone(),
            depth,
            kind: TreeRowKind::Directory,
            name: self.directory.name.clone(),
            expanded: self.expanded,
            selected: matches!(selected, TreeSelection::Directory(id) if id == &self.directory.id),
        });

        if !self.expanded {
            return;
        }

        for directory in &self.directories {
            directory.append_rows(depth + 1, selected, rows);
        }

        for note in &self.notes {
            rows.push(TreeRow {
                id: note.id.clone(),
                depth: depth + 1,
                kind: TreeRowKind::Note,
                name: note.name.clone(),
                expanded: false,
                selected: matches!(selected, TreeSelection::Note(id) if id == &note.id),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn demo_workspace_snapshot_covers_tree_and_note_content() {
        let workspace = WorkspaceModel::demo()
            .await
            .expect("demo workspace should open");

        let expected = [
            "selected: note:GPUI migration overview",
            "tree:",
            "  v Notes",
            "    > Glues GUI",
            "    > Testing",
            "  * - GPUI migration overview",
            "tabs:",
            "  * GPUI migration overview",
            "viewer:",
            "  note: GPUI migration overview",
            "  content:",
            "    # GPUI migration overview",
            "    ",
            "    Current milestone: read-only GUI over real backends.",
            "    ",
            "    Backend selection happens inside the GUI.",
            "    ",
            "    The old core state machine is not used by this frontend.",
        ]
        .join("\n");

        assert_eq!(workspace.snapshot_workspace(), expected);
    }

    #[tokio::test]
    async fn expanding_directory_loads_children_lazily() {
        let mut workspace = WorkspaceModel::demo()
            .await
            .expect("demo workspace should open");
        let gui_id = workspace
            .root
            .directories
            .iter()
            .find(|directory| directory.directory.name == "Glues GUI")
            .expect("gui directory should exist")
            .directory
            .id
            .clone();

        workspace
            .toggle_directory(gui_id)
            .await
            .expect("directory should expand");

        assert!(workspace.snapshot_workspace().contains("- Backend opening"));
    }

    #[tokio::test]
    async fn opened_note_directory_path_tracks_parent_directories() {
        let mut workspace = WorkspaceModel::demo()
            .await
            .expect("demo workspace should open");
        let gui_id = workspace
            .root
            .directories
            .iter()
            .find(|directory| directory.directory.name == "Glues GUI")
            .expect("gui directory should exist")
            .directory
            .id
            .clone();

        workspace
            .toggle_directory(gui_id)
            .await
            .expect("directory should expand");

        let note_id = workspace
            .root
            .find_note_by_name("Backend opening")
            .expect("backend note should exist")
            .id
            .clone();
        workspace
            .open_note(note_id)
            .await
            .expect("note should open");

        assert_eq!(
            workspace.opened_note_directory_path(),
            vec!["Notes".to_owned(), "Glues GUI".to_owned()]
        );
    }

    #[tokio::test]
    async fn opening_existing_note_reuses_tab() {
        let mut workspace = WorkspaceModel::demo()
            .await
            .expect("demo workspace should open");
        let overview_id = workspace
            .active_note()
            .expect("overview should be active")
            .note
            .id
            .clone();

        workspace
            .open_note(overview_id)
            .await
            .expect("opening an active note should succeed");

        assert_eq!(workspace.open_tabs().len(), 1);
        assert_eq!(workspace.active_tab_index(), Some(0));
    }

    #[tokio::test]
    async fn close_active_tab_selects_next_available_tab() {
        let mut workspace = WorkspaceModel::demo()
            .await
            .expect("demo workspace should open");
        let overview_id = workspace
            .active_note()
            .expect("overview should be active")
            .note
            .id
            .clone();
        let gui_id = workspace
            .root
            .directories
            .iter()
            .find(|directory| directory.directory.name == "Glues GUI")
            .expect("gui directory should exist")
            .directory
            .id
            .clone();

        workspace
            .toggle_directory(gui_id)
            .await
            .expect("directory should expand");

        let backend_note_id = workspace
            .root
            .find_note_by_name("Backend opening")
            .expect("backend note should exist")
            .id
            .clone();
        workspace
            .open_note(backend_note_id.clone())
            .await
            .expect("backend note should open");

        workspace
            .close_note_tab(backend_note_id)
            .expect("active backend tab should close");

        assert_eq!(workspace.open_tabs().len(), 1);
        assert_eq!(workspace.active_tab_index(), Some(0));
        assert_eq!(
            workspace
                .active_note()
                .expect("overview should remain active")
                .note
                .id,
            overview_id
        );
    }

    #[tokio::test]
    async fn closing_last_tab_leaves_empty_viewer() {
        let mut workspace = WorkspaceModel::demo()
            .await
            .expect("demo workspace should open");
        let overview_id = workspace
            .active_note()
            .expect("overview should be active")
            .note
            .id
            .clone();

        workspace
            .close_note_tab(overview_id)
            .expect("last tab should close");

        assert!(workspace.open_tabs().is_empty());
        assert_eq!(workspace.active_tab_index(), None);
        assert!(workspace.active_note().is_none());
    }

    #[tokio::test]
    async fn moving_tab_before_target_reorders_without_changing_active_note() {
        let mut workspace = workspace_with_three_open_tabs().await;
        let active_note_id = workspace
            .active_note()
            .expect("visual review should be active")
            .note
            .id
            .clone();
        let backend_note_id = workspace.open_tabs()[1].note.id.clone();

        workspace
            .move_note_tab_before(backend_note_id, 0)
            .expect("tab should move before first tab");

        assert_eq!(
            tab_names(&workspace),
            vec![
                "Backend opening",
                "GPUI migration overview",
                "Visual review"
            ]
        );
        assert_eq!(
            workspace
                .active_note()
                .expect("active note should remain selected")
                .note
                .id,
            active_note_id
        );
    }

    #[tokio::test]
    async fn moving_tab_to_end_reorders_without_changing_active_note() {
        let mut workspace = workspace_with_three_open_tabs().await;
        let active_note_id = workspace
            .active_note()
            .expect("visual review should be active")
            .note
            .id
            .clone();
        let overview_note_id = workspace.open_tabs()[0].note.id.clone();

        workspace
            .move_note_tab_to_end(overview_note_id)
            .expect("tab should move to end");

        assert_eq!(
            tab_names(&workspace),
            vec![
                "Backend opening",
                "Visual review",
                "GPUI migration overview"
            ]
        );
        assert_eq!(
            workspace
                .active_note()
                .expect("active note should remain selected")
                .note
                .id,
            active_note_id
        );
    }

    async fn workspace_with_three_open_tabs() -> WorkspaceModel {
        let mut workspace = WorkspaceModel::demo()
            .await
            .expect("demo workspace should open");

        for directory_name in ["Glues GUI", "Testing"] {
            let directory_id = workspace
                .root
                .directories
                .iter()
                .find(|directory| directory.directory.name == directory_name)
                .unwrap_or_else(|| panic!("{directory_name} directory should exist"))
                .directory
                .id
                .clone();

            workspace
                .toggle_directory(directory_id)
                .await
                .unwrap_or_else(|_| panic!("{directory_name} directory should expand"));
        }

        for note_name in ["Backend opening", "Visual review"] {
            let note_id = workspace
                .root
                .find_note_by_name(note_name)
                .unwrap_or_else(|| panic!("{note_name} note should exist"))
                .id
                .clone();

            workspace
                .open_note(note_id)
                .await
                .unwrap_or_else(|_| panic!("{note_name} note should open"));
        }

        workspace
    }

    fn tab_names(workspace: &WorkspaceModel) -> Vec<&str> {
        workspace
            .open_tabs()
            .iter()
            .map(|tab| tab.note.name.as_str())
            .collect()
    }
}
