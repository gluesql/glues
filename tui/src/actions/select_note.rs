use {
    super::sub_actions::update_statusbar,
    crate::traits::*,
    cursive::Cursive,
    glues_core::{data::Note, Event},
};

pub fn select_note(siv: &mut Cursive, note: Note) {
    siv.glues().dispatch(Event::SelectNote(note)).log_unwrap();

    update_statusbar(siv);
}
