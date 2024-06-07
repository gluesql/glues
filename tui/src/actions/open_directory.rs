use {
    crate::{traits::*, views::note_tree::directory::item_list::render_item_list, Node},
    cursive::Cursive,
    glues_core::types::DirectoryId,
};

pub fn open_directory(siv: &mut Cursive, directory_id: &DirectoryId) {
    siv.glues().open_directory(directory_id.clone());

    let item_list = render_item_list(siv, directory_id.clone());
    Node::note_tree()
        .directory(directory_id)
        .find(siv)
        .add_child(item_list);
}
