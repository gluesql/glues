mod caret;
mod note_list;

use {
    super::NoteTreeNode,
    crate::node::{NodePath, ViewFinder},
    caret::CaretNode,
    cursive::views::LinearLayout,
    glues_core::types::DirectoryId,
    note_list::NoteListNode,
};

pub struct DirectoryNode<'a> {
    parent: NoteTreeNode,
    directory_id: &'a DirectoryId,
}

impl<'a> DirectoryNode<'a> {
    pub fn new(parent: NoteTreeNode, directory_id: &'a DirectoryId) -> Self {
        DirectoryNode {
            parent,
            directory_id,
        }
    }

    pub fn caret(&'a self) -> CaretNode<'a> {
        CaretNode::new(self)
    }

    pub fn note_list(&'a self) -> NoteListNode<'a> {
        NoteListNode::new(self)
    }
}

impl<'a> NodePath for DirectoryNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("directory");
        path.push(self.directory_id);
        path
    }
}

impl<'a> ViewFinder<LinearLayout> for DirectoryNode<'a> {}
