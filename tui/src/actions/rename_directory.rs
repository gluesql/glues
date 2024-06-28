use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
    glues_core::data::Directory,
};

pub fn rename_directory(siv: &mut Cursive, directory: &Directory, new_name: &str) {
    let msg = format!(
        "[actions::rename_directory] directory_id: {}, rename to {}",
        directory.id, new_name,
    );
    log(&msg);

    // data
    siv.glues()
        .db
        .rename_directory(directory.id.clone(), new_name.to_owned())
        .log_unwrap();

    // ui
    Node::note_tree()
        .directory(&directory.id)
        .name_button()
        .find(siv)
        .set_label_raw(new_name);
}
