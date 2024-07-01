use {
    crate::{traits::*, views::note_tree::directory::item_list::render_item_list, Node},
    cursive::Cursive,
    glues_core::{data::Note, transition::OpenDirectory, types::DirectoryId, Event},
};

pub fn open_directory(siv: &mut Cursive, directory_id: &DirectoryId) {
    let OpenDirectory { notes, directories } =
        siv.dispatch(Event::OpenDirectory(directory_id.clone()));

    let directories = directories.to_vec();
    let notes: Vec<Note> = notes.to_vec();

    let directory_node = Node::note_tree().directory(directory_id);
    directory_node.caret().find(siv).set_content("▾ ");
    directory_node.find(siv).add_child(render_item_list(
        siv,
        directory_id.clone(),
        directories,
        notes,
    ));
}
