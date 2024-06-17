use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
    glues_core::data::Directory,
};

pub fn remove_directory(siv: &mut Cursive, directory: &Directory) {
    let msg = format!(
        "[actions::remove_directory] directory_id: {} / {}",
        directory.id, directory.name
    );
    log(&msg);

    // data
    siv.glues()
        .remove_directory(directory.id.clone())
        .log_unwrap();

    // ui - directory
    let mut container = if siv.glues().root_id == directory.id {
        Node::note_tree().note_list().find(siv)
    } else {
        Node::note_tree()
            .directory(&directory.parent_id)
            .note_list()
            .find(siv)
    };

    let i = container
        .find_child_from_name(&Node::note_tree().directory(&directory.id).name())
        .log_expect("[actions::remove_directory] directory does not exist");

    container.remove_child(i);
}
