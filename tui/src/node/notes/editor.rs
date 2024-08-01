mod content;
mod name_button;
mod panel;
mod status;

use {
    crate::node::{NodePath, ViewFinder},
    content::ContentNode,
    cursive::views::StackView,
    name_button::NameButtonNode,
    panel::PanelNode,
    status::StatusNode,
};

#[derive(Clone)]
pub struct EditorNode;

const PATH: &str = "editor";

impl EditorNode {
    pub fn panel(&self) -> PanelNode<'_> {
        PanelNode::new(self)
    }

    pub fn name_button(&self) -> NameButtonNode<'_> {
        NameButtonNode::new(self)
    }

    pub fn content(&self) -> ContentNode<'_> {
        ContentNode::new(self)
    }

    pub fn status(&self) -> StatusNode<'_> {
        StatusNode::new(self)
    }
}

impl NodePath for EditorNode {
    fn get_path(&self) -> Vec<&str> {
        vec![PATH]
    }
}

impl ViewFinder<StackView> for EditorNode {}
