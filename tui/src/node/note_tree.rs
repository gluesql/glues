mod directory;
mod note;
mod note_list;

use {
    crate::node::NodePath,
    directory::DirectoryNode,
    glues_core::types::{DirectoryId, NoteId},
    note::NoteNode,
    note_list::NoteListNode,
};

#[derive(Clone)]
pub struct NoteTreeNode;

const PATH: &str = "note_tree";

impl NoteTreeNode {
    pub fn note_list(&self) -> NoteListNode {
        NoteListNode::new(self.clone())
    }

    pub fn directory<'a>(&self, directory_id: &'a DirectoryId) -> DirectoryNode<'a> {
        DirectoryNode::new(self.clone(), directory_id)
    }

    pub fn note<'a>(&self, note_id: &'a NoteId) -> NoteNode<'a> {
        NoteNode::new(self.clone(), note_id)
    }
}

impl NodePath for NoteTreeNode {
    fn get_path(&self) -> Vec<&str> {
        vec![PATH]
    }
}
