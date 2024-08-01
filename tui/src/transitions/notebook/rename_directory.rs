use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::data::Directory,
};

pub fn rename_directory(siv: &mut Cursive, directory: Directory) {
    log!(
        "[transition::rename_directory] directory_id: {}, rename to {}",
        directory.id,
        directory.name,
    );

    // ui
    Node::notebook()
        .note_tree()
        .directory(&directory.id)
        .name_button()
        .find(siv)
        .set_label_raw(directory.name);
}
