use {
    crate::views::notes::note_tree::note::more_actions::render_more_actions, cursive::Cursive,
    glues_core::data::Note,
};

pub fn show_note_actions(siv: &mut Cursive, note: Note) {
    let dialog = render_more_actions(note);
    siv.add_layer(dialog);
}
