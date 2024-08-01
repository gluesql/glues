use {
    super::DirectoryNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::TextView,
};

pub struct CaretNode<'a> {
    parent: &'a DirectoryNode<'a>,
}

impl<'a> CaretNode<'a> {
    pub fn new(parent: &'a DirectoryNode<'a>) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for CaretNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("caret");
        path
    }
}

impl<'a> ViewFinder<TextView> for CaretNode<'a> {}
