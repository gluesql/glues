use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::types::DirectoryId,
};

pub fn close_directory(siv: &mut Cursive, directory_id: &DirectoryId) {
    siv.glues().close_directory(directory_id);

    Node::note_tree()
        .directory(directory_id)
        .find(siv)
        .remove_child(1);
}
