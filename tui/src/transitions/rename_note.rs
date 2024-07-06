use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
    glues_core::types::NoteId,
};

pub fn rename_note(siv: &mut Cursive, id: NoteId, new_name: String) {
    let msg = format!(
        "[transition::rename_note] note_id: {}, rename to {}",
        id, new_name,
    );
    log(&msg);

    // ui
    Node::note_tree()
        .note(&id)
        .name_button()
        .find(siv)
        .set_label_raw(new_name);
}
