mod description;
mod shortcuts;

use {crate::node::NodePath, description::DescriptionNode, shortcuts::ShortcutsNode};

pub struct StatusbarNode;

impl StatusbarNode {
    pub fn description(&self) -> DescriptionNode<'_> {
        DescriptionNode::new(self)
    }

    pub fn shortcuts(&self) -> ShortcutsNode<'_> {
        ShortcutsNode::new(self)
    }
}

impl NodePath for StatusbarNode {
    fn get_path(&self) -> Vec<&str> {
        vec!["statusbar"]
    }
}
