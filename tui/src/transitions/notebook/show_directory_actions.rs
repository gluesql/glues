use {
    crate::views::notebook::note_tree::directory::more_actions::render_more_actions,
    cursive::Cursive, glues_core::data::Directory,
};

pub fn show_directory_actions(siv: &mut Cursive, directory: Directory) {
    let dialog = render_more_actions(directory);
    siv.add_layer(dialog);
}
