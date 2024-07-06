use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
    glues_core::types::DirectoryId,
};

pub fn rename_directory(siv: &mut Cursive, id: DirectoryId, new_name: String) {
    let msg = format!(
        "[transition::rename_directory] directory_id: {}, rename to {}",
        id, new_name,
    );
    log(&msg);

    // ui
    Node::note_tree()
        .directory(&id)
        .name_button()
        .find(siv)
        .set_label_raw(new_name);
}
