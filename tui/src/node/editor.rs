mod content;
mod panel;

use {
    crate::node::{NodePath, ViewFinder},
    content::ContentNode,
    cursive::views::StackView,
    panel::PanelNode,
};

#[derive(Clone)]
pub struct EditorNode;

const PATH: &str = "editor";

impl EditorNode {
    pub fn panel(&self) -> PanelNode<'_> {
        PanelNode::new(self)
    }

    pub fn content(&self) -> ContentNode<'_> {
        ContentNode::new(self)
    }
}

impl NodePath for EditorNode {
    fn get_path(&self) -> Vec<&str> {
        vec![PATH]
    }
}

impl<'a> ViewFinder<StackView> for EditorNode {}
