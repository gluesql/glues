use {
    super::EditorNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::Button,
};

pub struct NameButtonNode<'a> {
    parent: &'a EditorNode,
}

impl<'a> NameButtonNode<'a> {
    pub fn new(parent: &'a EditorNode) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for NameButtonNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("name");
        path
    }
}

impl<'a> ViewFinder<Button> for NameButtonNode<'a> {}
