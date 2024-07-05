use {
    super::StatusbarNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::TextView,
};

pub struct ShortcutsNode<'a> {
    parent: &'a StatusbarNode,
}

impl<'a> ShortcutsNode<'a> {
    pub fn new(parent: &'a StatusbarNode) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for ShortcutsNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("shortcuts");
        path
    }
}

impl<'a> ViewFinder<TextView> for ShortcutsNode<'a> {}
