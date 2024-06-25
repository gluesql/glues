use {
    crate::node::{NodePath, ViewFinder},
    cursive::views::StackView,
};

pub struct BodyNode;

impl NodePath for BodyNode {
    fn get_path(&self) -> Vec<&str> {
        vec!["body"]
    }
}

impl ViewFinder<StackView> for BodyNode {}
