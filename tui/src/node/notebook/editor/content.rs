use {
    super::EditorNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::TextArea,
};

pub struct ContentNode<'a> {
    parent: &'a EditorNode,
}

impl<'a> ContentNode<'a> {
    pub fn new(parent: &'a EditorNode) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for ContentNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("content");
        path
    }
}

impl<'a> ViewFinder<TextArea> for ContentNode<'a> {}
