use {
    super::EditorNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::TextView,
};

pub struct StatusNode<'a> {
    parent: &'a EditorNode,
}

impl<'a> StatusNode<'a> {
    pub fn new(parent: &'a EditorNode) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for StatusNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("status");
        path
    }
}

impl<'a> ViewFinder<TextView> for StatusNode<'a> {}
