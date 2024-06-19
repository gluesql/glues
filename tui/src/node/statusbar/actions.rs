use {
    super::StatusbarNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::TextView,
};

pub struct ActionsNode<'a> {
    parent: &'a StatusbarNode,
}

impl<'a> ActionsNode<'a> {
    pub fn new(parent: &'a StatusbarNode) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for ActionsNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("actions");
        path
    }
}

impl<'a> ViewFinder<TextView> for ActionsNode<'a> {}
