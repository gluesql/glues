use crate::{
    data::{Directory, Note},
    types::{DirectoryId, Id},
};

#[derive(Clone)]
pub struct DirectoryItem {
    pub directory: Directory,
    pub children: Option<DirectoryItemChildren>,
}

#[derive(Clone)]
pub struct DirectoryItemChildren {
    pub directories: Vec<DirectoryItem>,
    pub notes: Vec<Note>,
}

impl DirectoryItem {
    pub fn find(&self, id: &DirectoryId) -> Option<&DirectoryItem> {
        if &self.directory.id == id {
            return Some(self);
        }

        self.children
            .as_ref()?
            .directories
            .iter()
            .filter_map(|item| item.find(id))
            .next()
    }

    pub fn find_mut(&mut self, id: &DirectoryId) -> Option<&mut DirectoryItem> {
        if &self.directory.id == id {
            return Some(self);
        }

        self.children
            .as_mut()?
            .directories
            .iter_mut()
            .filter_map(|item| item.find_mut(id))
            .next()
    }

    fn tree_items(&self) -> Vec<TreeItem> {
        let mut items = vec![TreeItem::Directory(&self.directory)];

        if let Some(children) = &self.children {
            for item in &children.directories {
                items.extend(item.tree_items());
            }

            for note in &children.notes {
                items.push(TreeItem::Note(note));
            }
        }

        items
    }

    pub fn find_prev(&self, id: &Id) -> Option<TreeItem> {
        let tree_items = self.tree_items();
        let i = tree_items.iter().position(|item| match item {
            TreeItem::Directory(directory) => &directory.id == id,
            TreeItem::Note(note) => &note.id == id,
        })?;

        tree_items.get(if i > 0 { i - 1 } else { 0 }).cloned()
    }

    pub fn find_next(&self, id: &Id) -> Option<TreeItem> {
        let tree_items = self.tree_items();
        let i = tree_items.iter().position(|item| match item {
            TreeItem::Directory(directory) => &directory.id == id,
            TreeItem::Note(note) => &note.id == id,
        })?;

        tree_items.get(i + 1).cloned()
    }
}

#[derive(Clone)]
pub enum TreeItem<'a> {
    Note(&'a Note),
    Directory(&'a Directory),
}
