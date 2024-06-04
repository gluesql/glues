mod directory;
mod note;

use {
    directory::DirectoryNode,
    glues_core::types::{DirectoryId, NoteId},
    note::NoteNode,
};

pub struct NoteTreeNode;

const PATH: &str = "note_tree";

impl NoteTreeNode {
    pub fn directory<'a>(&self, directory_id: &'a DirectoryId) -> DirectoryNode<'a> {
        DirectoryNode::new(vec![PATH], directory_id)
    }

    pub fn note<'a>(&self, note_id: &'a NoteId) -> NoteNode<'a> {
        NoteNode::new(vec![PATH], note_id)
    }
}
