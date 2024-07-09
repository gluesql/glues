use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
    glues_core::data::Directory,
};

pub fn rename_directory(siv: &mut Cursive, directory: Directory) {
    let msg = format!(
        "[transition::rename_directory] directory_id: {}, rename to {}",
        directory.id, directory.name,
    );
    log(&msg);

    // ui
    Node::note_tree()
        .directory(&directory.id)
        .name_button()
        .find(siv)
        .set_label_raw(directory.name);
}
