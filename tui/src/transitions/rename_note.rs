use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
    glues_core::data::Note,
};

pub fn rename_note(siv: &mut Cursive, note: Note) {
    let msg = format!(
        "[transition::rename_note] note_id: {}, rename to {}",
        note.id, note.name,
    );
    log(&msg);

    // ui
    Node::note_tree()
        .note(&note.id)
        .name_button()
        .find(siv)
        .set_label_raw(note.name);
}
