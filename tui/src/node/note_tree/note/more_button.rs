use {
    super::NoteNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::Button,
};

pub struct MoreButtonNode<'a> {
    parent: &'a NoteNode<'a>,
}

impl<'a> MoreButtonNode<'a> {
    pub fn new(parent: &'a NoteNode<'a>) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for MoreButtonNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("more_button");
        path
    }
}

impl<'a> ViewFinder<Button> for MoreButtonNode<'a> {}
