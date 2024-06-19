use {
    super::StatusbarNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::TextView,
};

pub struct DescriptionNode<'a> {
    parent: &'a StatusbarNode,
}

impl<'a> DescriptionNode<'a> {
    pub fn new(parent: &'a StatusbarNode) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for DescriptionNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("description");
        path
    }
}

impl<'a> ViewFinder<TextView> for DescriptionNode<'a> {}
