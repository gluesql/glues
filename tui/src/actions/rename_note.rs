use {
    crate::{logger::log, traits::*},
    cursive::Cursive,
    glues_core::data::Note,
};

pub fn rename_note(siv: &mut Cursive, note: &Note) {
    let msg = format!(
        "[actions::rename_note] TODO directory_id: {}, note_id: {}",
        note.directory_id, note.id,
    );
    log(&msg);

    siv.alert("WIP".to_owned(), |_| {});
}
