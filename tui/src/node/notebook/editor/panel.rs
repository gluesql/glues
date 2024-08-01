use {
    super::EditorNode,
    crate::node::{NodePath, ViewFinder},
    cursive::views::{NamedView, PaddedView, Panel, StackView},
};

pub struct PanelNode<'a> {
    parent: &'a EditorNode,
}

impl<'a> PanelNode<'a> {
    pub fn new(parent: &'a EditorNode) -> Self {
        Self { parent }
    }
}

impl<'a> NodePath for PanelNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("panel");
        path
    }
}

impl<'a> ViewFinder<Panel<PaddedView<NamedView<StackView>>>> for PanelNode<'a> {}
