use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::data::{Directory, Note},
};

pub fn remove_note(siv: &mut Cursive, note: Note, selected_directory: Directory) {
    log!(
        "[transitions::remove_note] directory_id: {}, note_id: {}",
        note.directory_id,
        note.id,
    );

    let mut container = Node::notebook()
        .note_tree()
        .directory(&note.directory_id)
        .find(siv);

    let i = container
        .find_child_from_name(&Node::notebook().note_tree().note(&note.id).name())
        .log_expect("[transitions::remove_note] note does not exist");

    container.remove_child(i);

    siv.focus_on_next_tick(
        Node::notebook()
            .note_tree()
            .directory(&selected_directory.id)
            .name_button()
            .name(),
    );
}
