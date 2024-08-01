use {
    super::DirectoryNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::LinearLayout,
};

pub struct NoteListNode<'a> {
    parent: &'a DirectoryNode<'a>,
}

impl<'a> NoteListNode<'a> {
    pub fn new(parent: &'a DirectoryNode<'a>) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for NoteListNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("note_list");
        path
    }
}

impl<'a> ViewFinder<LinearLayout> for NoteListNode<'a> {}
