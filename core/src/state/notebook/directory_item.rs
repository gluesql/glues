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

    pub fn rename_directory(&mut self, target: &Directory) -> Option<()> {
        let directory_item = self.find_mut(&target.id)?;
        directory_item.directory.name = target.name.clone();

        Some(())
    }

    pub fn rename_note(&mut self, target: &Note) -> Option<()> {
        let directory_item = self.find_mut(&target.directory_id)?;
        for note in directory_item.children.as_mut()?.notes.iter_mut() {
            if note.id == target.id {
                note.name = target.name.clone();
                break;
            }
        }

        Some(())
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

    pub(crate) fn tree_items(&self, depth: usize) -> Vec<TreeItem<'_>> {
        let mut items = vec![TreeItem {
            id: &self.directory.id,
            name: &self.directory.name,
            depth,
        }];

        if let Some(children) = &self.children {
            for item in &children.directories {
                items.extend(item.tree_items(depth + 1));
            }

            for note in &children.notes {
                items.push(TreeItem {
                    id: &note.id,
                    name: &note.name,
                    depth: depth + 1,
                });
            }
        }

        items
    }
}

pub struct TreeItem<'a> {
    pub id: &'a Id,
    pub name: &'a str,
    pub depth: usize,
}
