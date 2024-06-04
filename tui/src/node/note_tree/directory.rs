mod caret;
mod note_list;

use {
    crate::node::LeafNode, caret::CaretNode, cursive::views::LinearLayout,
    glues_core::types::DirectoryId, note_list::NoteListNode,
};

pub struct DirectoryNode<'a> {
    path: Vec<&'a str>,
}

impl<'a> DirectoryNode<'a> {
    pub fn new(mut path: Vec<&'a str>, directory_id: &'a DirectoryId) -> Self {
        path.push("directory");
        path.push(directory_id);

        DirectoryNode { path }
    }

    pub fn caret(&'a self) -> CaretNode<'a> {
        CaretNode::new(self)
    }

    pub fn note_list(&'a self) -> NoteListNode<'a> {
        NoteListNode::new(self)
    }
}

impl<'a> LeafNode<LinearLayout> for DirectoryNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        self.path.clone()
    }
}
