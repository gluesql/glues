use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::{data::Directory, NotebookEvent},
};

pub fn add_directory(siv: &mut Cursive, directory: Directory) {
    siv.dispatch(NotebookEvent::OpenDirectory(directory.parent_id.clone()));
    siv.focus_on_next_tick(
        Node::notebook()
            .note_tree()
            .directory(&directory.id)
            .name_button()
            .name(),
    );
}
