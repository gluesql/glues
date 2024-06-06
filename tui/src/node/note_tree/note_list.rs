use {
    super::NoteTreeNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::LinearLayout,
};

pub struct NoteListNode {
    parent: NoteTreeNode,
}

impl NoteListNode {
    pub fn new(parent: NoteTreeNode) -> Self {
        Self { parent }
    }
}

impl NodePath for NoteListNode {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("note_list");
        path
    }
}

impl ViewFinder<LinearLayout> for NoteListNode {}
