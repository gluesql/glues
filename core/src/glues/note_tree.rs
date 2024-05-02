use {
    crate::{types::DirectoryId, Glues},
    std::collections::HashSet,
};

pub type NoteTree = HashSet<DirectoryId>;

impl Glues {
    pub fn check_opened(&self, directory_id: &DirectoryId) -> bool {
        self.note_tree.contains(directory_id)
    }

    pub fn open_directory(&mut self, directory_id: DirectoryId) {
        self.note_tree.insert(directory_id);
    }

    pub fn close_directory(&mut self, directory_id: &DirectoryId) {
        self.note_tree.remove(directory_id);
    }
}
