use {super::NoteNode, crate::node::LeafNode, cursive::views::Button};

pub struct NameButtonNode<'a> {
    parent: &'a NoteNode<'a>,
}

impl<'a> NameButtonNode<'a> {
    pub fn new(parent: &'a NoteNode<'a>) -> Self {
        Self { parent }
    }
}

impl<'a> LeafNode<Button> for NameButtonNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("name_button");
        path
    }
}
