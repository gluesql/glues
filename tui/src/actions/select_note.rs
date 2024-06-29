use {
    super::sub_actions::update_statusbar,
    crate::traits::*,
    cursive::Cursive,
    glues_core::{data::Note, state::note_tree::NoteTreeState},
};

pub fn select_note(siv: &mut Cursive, note: Note) {
    siv.state_mut::<NoteTreeState>().select_note(note);

    update_statusbar(siv);
}
