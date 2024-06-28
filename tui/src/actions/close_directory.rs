use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::types::DirectoryId,
};

pub fn close_directory(siv: &mut Cursive, directory_id: &DirectoryId) {
    siv.glues().db.close_directory(directory_id);

    let directory_node = Node::note_tree().directory(directory_id);
    directory_node.caret().find(siv).set_content("▸ ");
    directory_node.find(siv).remove_child(1);
}
