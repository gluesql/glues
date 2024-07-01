use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::{types::DirectoryId, Event},
};

pub fn close_directory(siv: &mut Cursive, directory_id: &DirectoryId) {
    siv.dispatch::<()>(Event::CloseDirectory(directory_id.clone()));

    let directory_node = Node::note_tree().directory(directory_id);
    directory_node.caret().find(siv).set_content("▸ ");
    directory_node.find(siv).remove_child(1);
}
