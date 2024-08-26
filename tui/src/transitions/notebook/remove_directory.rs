use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::data::Directory,
};

pub fn remove_directory(siv: &mut Cursive, directory: Directory, selected_directory: Directory) {
    log!(
        "[transitions::remove_directory] directory_id: {} / {}",
        directory.id,
        directory.name
    );

    // ui - directory
    let mut container = Node::notebook()
        .note_tree()
        .directory(&directory.parent_id)
        .find(siv);

    let i = container
        .find_child_from_name(&Node::notebook().note_tree().directory(&directory.id).name())
        .log_expect("[transitions::remove_directory] directory does not exist");

    container.remove_child(i);

    siv.focus_on_next_tick(
        Node::notebook()
            .note_tree()
            .directory(&selected_directory.id)
            .name_button()
            .name(),
    );
}
