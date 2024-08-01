use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::data::Note,
};

pub fn select_note(siv: &mut Cursive, note: Note) {
    Node::notebook()
        .note_tree()
        .note(&note.id)
        .name_button()
        .find(siv)
        .enable();

    siv.focus_on_next_tick(
        Node::notebook()
            .note_tree()
            .note(&note.id)
            .name_button()
            .name(),
    );
}
