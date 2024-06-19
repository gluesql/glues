mod actions;
mod description;

use {crate::node::NodePath, actions::ActionsNode, description::DescriptionNode};

pub struct StatusbarNode;

impl StatusbarNode {
    pub fn description(&self) -> DescriptionNode<'_> {
        DescriptionNode::new(self)
    }

    pub fn actions(&self) -> ActionsNode<'_> {
        ActionsNode::new(self)
    }
}

impl NodePath for StatusbarNode {
    fn get_path(&self) -> Vec<&str> {
        vec!["statusbar"]
    }
}
