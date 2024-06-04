use {super::DirectoryNode, crate::node::LeafNode, cursive::views::LinearLayout};

pub struct NoteListNode<'a> {
    parent: &'a DirectoryNode<'a>,
}

impl<'a> NoteListNode<'a> {
    pub fn new(parent: &'a DirectoryNode<'a>) -> Self {
        Self { parent }
    }
}

impl<'a> LeafNode<LinearLayout> for NoteListNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("note_list");
        path
    }
}
