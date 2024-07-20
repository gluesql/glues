use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::data::Note,
};

pub fn view_mode(siv: &mut Cursive, note: Note) {
    Node::editor().content().find(siv).disable();

    siv.focus_name(&Node::note_tree().note(&note.id).name_button().name())
        .log_unwrap();
}
