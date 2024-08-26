use crate::{
    data::{Directory, Note},
    types::{DirectoryId, Id},
};

#[derive(Clone, Debug)]
pub struct DirectoryItem {
    pub directory: Directory,
    pub children: Option<DirectoryItemChildren>,
}

#[derive(Clone, Debug)]
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

    pub fn remove_note(&mut self, target: &Note) -> Option<&Directory> {
        let directory_item = self.find_mut(&target.directory_id)?;
        directory_item
            .children
            .as_mut()?
            .notes
            .retain_mut(|note| note.id != target.id);

        Some(&directory_item.directory)
    }

    pub fn remove_directory(&mut self, target: &Directory) -> Option<&Directory> {
        let directory_item = self.find_mut(&target.parent_id)?;
        directory_item
            .children
            .as_mut()?
            .directories
            .retain_mut(|item| item.directory.id != target.id);

        Some(&directory_item.directory)
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
