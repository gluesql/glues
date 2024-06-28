use {
    super::sub_actions::update_statusbar,
    crate::traits::*,
    cursive::Cursive,
    glues_core::{state::note_tree::DirectoryItem, Event},
};

pub fn select_directory(siv: &mut Cursive, directory_item: DirectoryItem) {
    siv.glues()
        .dispatch(Event::SelectDirectory(directory_item))
        .log_unwrap();

    update_statusbar(siv);
}
