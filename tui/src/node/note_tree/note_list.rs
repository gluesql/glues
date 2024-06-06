use {crate::node::LeafNode, cursive::views::LinearLayout};

pub struct NoteListNode {
    path: Vec<&'static str>,
}

impl NoteListNode {
    pub fn new(path: Vec<&'static str>) -> Self {
        Self { path }
    }
}

impl LeafNode<LinearLayout> for NoteListNode {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.path.clone();

        path.push("note_list");
        path
    }
}
