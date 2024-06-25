mod body;
mod note_tree;
mod statusbar;

use {
    crate::traits::CursiveExt,
    body::BodyNode,
    cursive::{view::View, views::ViewRef, Cursive},
    note_tree::NoteTreeNode,
    statusbar::StatusbarNode,
};

pub struct Node {}

impl Node {
    pub fn body() -> BodyNode {
        BodyNode
    }

    pub fn note_tree() -> NoteTreeNode {
        NoteTreeNode
    }

    pub fn statusbar() -> StatusbarNode {
        StatusbarNode
    }
}

pub(crate) trait NodePath {
    fn get_path(&self) -> Vec<&str>;
}

pub(crate) trait ViewNamed: NodePath {
    fn name(&self) -> String {
        self.get_path().join("/")
    }
}

pub(crate) trait ViewFinder<V: View>: NodePath {
    fn name(&self) -> String {
        self.get_path().join("/")
    }

    fn find(&self, siv: &mut Cursive) -> ViewRef<V> {
        siv.find(&self.name())
    }
}
