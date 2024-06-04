mod note_tree;

use note_tree::NoteTreeNode;

use {
    crate::traits::CursiveExt,
    cursive::{view::View, views::ViewRef, Cursive},
};

pub struct Node {}

impl Node {
    pub fn note_tree() -> NoteTreeNode {
        NoteTreeNode {}
    }
}

pub(crate) trait LeafNode<V: View> {
    fn get_path(&self) -> Vec<&str>;

    fn name(&self) -> String {
        self.get_path().join("/")
    }

    fn find(&self, siv: &mut Cursive) -> ViewRef<V> {
        siv.find(&self.name())
    }
}
