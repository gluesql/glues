mod name_button;

use {
    super::NoteTreeNode,
    crate::node::{NodePath, ViewNamed},
    glues_core::types::NoteId,
    name_button::NameButtonNode,
};

pub struct NoteNode<'a> {
    parent: NoteTreeNode,
    note_id: &'a String,
}

impl<'a> NoteNode<'a> {
    pub fn new(parent: NoteTreeNode, note_id: &'a NoteId) -> Self {
        NoteNode { parent, note_id }
    }

    pub fn name_button(&'a self) -> NameButtonNode<'a> {
        NameButtonNode::new(self)
    }
}

impl<'a> NodePath for NoteNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();
        path.push("note");
        path.push(self.note_id);

        path
    }
}

impl<'a> ViewNamed for NoteNode<'a> {}
