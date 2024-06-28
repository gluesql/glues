use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
    glues_core::data::Note,
};

pub fn rename_note(siv: &mut Cursive, note: &Note, new_name: &str) {
    let msg = format!(
        "[actions::rename_note] note_id: {}, rename to {}",
        note.id, new_name,
    );
    log(&msg);

    // data
    siv.glues()
        .db
        .rename_note(note.id.clone(), new_name.to_owned())
        .log_unwrap();

    // ui
    Node::note_tree()
        .note(&note.id)
        .name_button()
        .find(siv)
        .set_label_raw(new_name);
}
