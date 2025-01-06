use glues_core::{
    data::{Directory, Note},
    types::Id,
};

#[derive(Clone)]
pub struct TreeItem {
    pub depth: usize,
    pub target: bool,
    pub selectable: bool,
    pub kind: TreeItemKind,
}

#[derive(Clone)]
pub enum TreeItemKind {
    Note { note: Note },
    Directory { directory: Directory, opened: bool },
}

impl TreeItem {
    pub fn is_directory(&self) -> bool {
        matches!(self.kind, TreeItemKind::Directory { .. })
    }

    pub fn id(&self) -> &Id {
        match &self.kind {
            TreeItemKind::Note { note, .. } => &note.id,
            TreeItemKind::Directory { directory, .. } => &directory.id,
        }
    }

    pub fn name(&self) -> String {
        match &self.kind {
            TreeItemKind::Note { note, .. } => &note.name,
            TreeItemKind::Directory { directory, .. } => &directory.name,
        }
        .clone()
    }
}
