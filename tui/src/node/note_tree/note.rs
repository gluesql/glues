mod more_button;
mod name_button;

use {
    crate::node::LeafNode, cursive::views::LinearLayout, glues_core::types::NoteId,
    more_button::MoreButtonNode, name_button::NameButtonNode,
};

pub struct NoteNode<'a> {
    path: Vec<&'a str>,
}

impl<'a> NoteNode<'a> {
    pub fn new(mut path: Vec<&'a str>, note_id: &'a NoteId) -> Self {
        path.push("note");
        path.push(note_id);

        NoteNode { path }
    }

    pub fn name_button(&'a self) -> NameButtonNode<'a> {
        NameButtonNode::new(self)
    }

    pub fn more_button(&'a self) -> MoreButtonNode<'a> {
        MoreButtonNode::new(self)
    }
}

impl<'a> LeafNode<LinearLayout> for NoteNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        self.path.clone()
    }
}
