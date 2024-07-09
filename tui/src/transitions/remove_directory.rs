use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
    glues_core::data::Directory,
};

pub fn remove_directory(siv: &mut Cursive, directory: Directory) {
    let msg = format!(
        "[transitions::remove_directory] directory_id: {} / {}",
        directory.id, directory.name
    );
    log(&msg);

    // ui - directory
    let mut container = Node::note_tree()
        .directory(&directory.parent_id)
        .note_list()
        .find(siv);

    let i = container
        .find_child_from_name(&Node::note_tree().directory(&directory.id).name())
        .log_expect("[transitions::remove_directory] directory does not exist");

    container.remove_child(i);
}
