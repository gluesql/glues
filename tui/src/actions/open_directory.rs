use {
    crate::{traits::*, views::note_tree::directory::item_list::render_item_list, Node},
    cursive::Cursive,
    glues_core::types::DirectoryId,
};

pub fn open_directory(siv: &mut Cursive, directory_id: &DirectoryId) {
    siv.glues().db.open_directory(directory_id.clone());

    let directory_node = Node::note_tree().directory(directory_id);
    directory_node.caret().find(siv).set_content("▾ ");
    directory_node
        .find(siv)
        .add_child(render_item_list(siv, directory_id.clone()));
}
