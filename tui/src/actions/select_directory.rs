use {
    super::sub_actions::update_statusbar,
    crate::traits::*,
    cursive::Cursive,
    glues_core::{state::note_tree::NoteTreeState, types::DirectoryId},
};

pub fn select_directory(siv: &mut Cursive, id: DirectoryId, name: String) {
    siv.state_mut::<NoteTreeState>().select_directory(id, name);

    update_statusbar(siv);
}
