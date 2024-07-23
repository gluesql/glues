use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::data::Note,
};

pub fn select_note(siv: &mut Cursive, note: Note) {
    Node::note_tree()
        .note(&note.id)
        .name_button()
        .find(siv)
        .enable();

    siv.cb_sink()
        .send(Box::new(move |siv| {
            siv.focus_name(&Node::note_tree().note(&note.id).name_button().name())
                .log_unwrap();
        }))
        .log_unwrap();
}
