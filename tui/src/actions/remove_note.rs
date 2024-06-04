use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
    glues_core::data::Note,
};

pub fn remove_note(siv: &mut Cursive, note: &Note) {
    let msg = format!(
        "[actions::remove_note] directory_id: {}, note_id: {}",
        note.directory_id, note.id,
    );
    log(&msg);

    // data
    siv.glues().remove_note(note.id.clone()).log_unwrap();

    // ui - directory
    let mut container = Node::note_tree()
        .directory(&note.directory_id)
        .note_list()
        .find(siv);

    let i = container
        .find_child_from_name(&Node::note_tree().note(&note.id).name())
        .log_expect("[actions::remove_note] note does not exist");

    container.remove_child(i);
}
