use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::data::Directory,
};

pub fn select_directory(siv: &mut Cursive, directory: Directory) {
    siv.focus_on_next_tick(
        Node::notebook()
            .note_tree()
            .directory(&directory.id)
            .name_button()
            .name(),
    );
}
