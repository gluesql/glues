use {
    super::NoteNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::Button,
};

pub struct NameButtonNode<'a> {
    parent: &'a NoteNode<'a>,
}

impl<'a> NameButtonNode<'a> {
    pub fn new(parent: &'a NoteNode<'a>) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for NameButtonNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("name_button");
        path
    }
}

impl<'a> ViewFinder<Button> for NameButtonNode<'a> {}
