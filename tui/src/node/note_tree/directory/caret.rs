use {super::DirectoryNode, crate::node::LeafNode, cursive::views::TextView};

pub struct CaretNode<'a> {
    parent: &'a DirectoryNode<'a>,
}

impl<'a> CaretNode<'a> {
    pub fn new(parent: &'a DirectoryNode<'a>) -> Self {
        Self { parent }
    }
}

impl<'a> LeafNode<TextView> for CaretNode<'a> {
    fn get_path(&self) -> Vec<&str> {
        let mut path = self.parent.get_path();

        path.push("caret");
        path
    }
}
