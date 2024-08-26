use {
    crate::{traits::*, views::notebook::note_tree::directory::item_list::render_item_list, Node},
    cursive::Cursive,
    glues_core::{data::Note, state::notebook::DirectoryItem, types::DirectoryId},
};

pub fn open_directory(
    siv: &mut Cursive,
    id: DirectoryId,
    notes: Vec<Note>,
    directories: Vec<DirectoryItem>,
) {
    let directory_node = Node::notebook().note_tree().directory(&id);
    directory_node.caret().find(siv).set_content("▾ ");

    let mut container = directory_node.find(siv);
    let n = container.len();
    for _ in 1..n {
        container.remove_child(1);
    }

    render_item_list(siv, &mut container, directories, notes);
}
