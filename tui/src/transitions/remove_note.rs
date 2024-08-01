use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::data::Note,
};

pub fn remove_note(siv: &mut Cursive, note: Note) {
    log!(
        "[transitions::remove_note] directory_id: {}, note_id: {}",
        note.directory_id,
        note.id,
    );

    let mut container = Node::notes()
        .note_tree()
        .directory(&note.directory_id)
        .note_list()
        .find(siv);

    let i = container
        .find_child_from_name(&Node::notes().note_tree().note(&note.id).name())
        .log_expect("[transitions::remove_note] note does not exist");

    container.remove_child(i);
}
