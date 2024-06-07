mod note_tree;

use {
    crate::traits::CursiveExt,
    cursive::{view::View, views::ViewRef, Cursive},
    note_tree::NoteTreeNode,
};

pub struct Node {}

impl Node {
    pub fn note_tree() -> NoteTreeNode {
        NoteTreeNode {}
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
