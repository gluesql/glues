mod content;
mod name_button;
mod panel;

use {
    crate::node::{NodePath, ViewFinder},
    content::ContentNode,
    cursive::views::StackView,
    name_button::NameButtonNode,
    panel::PanelNode,
};

#[derive(Clone)]
pub struct EditorNode;

const PATH: &str = "editor";

impl EditorNode {
    // TODO: deprecated, may be...
    pub fn panel(&self) -> PanelNode<'_> {
        PanelNode::new(self)
    }

    pub fn name_button(&self) -> NameButtonNode<'_> {
        NameButtonNode::new(self)
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

impl ViewFinder<StackView> for EditorNode {}
