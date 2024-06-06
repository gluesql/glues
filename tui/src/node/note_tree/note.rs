mod more_button;
mod name_button;

use {
    super::NoteTreeNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::LinearLayout,
    glues_core::types::NoteId,
    more_button::MoreButtonNode,
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

    pub fn more_button(&'a self) -> MoreButtonNode<'a> {
        MoreButtonNode::new(self)
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

impl<'a> ViewFinder<LinearLayout> for NoteNode<'a> {}
