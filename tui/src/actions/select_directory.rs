use {
    super::sub_actions::update_statusbar,
    crate::traits::*,
    cursive::Cursive,
    glues_core::state::note_tree::{DirectoryItem, NoteTreeState},
};

pub fn select_directory(siv: &mut Cursive, directory_item: DirectoryItem) {
    siv.state_mut::<NoteTreeState>()
        .select_directory(directory_item);

    update_statusbar(siv);
}
