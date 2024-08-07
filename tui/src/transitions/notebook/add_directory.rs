use {
    crate::{traits::*, views::notebook::note_tree::directory::render_directory, Node},
    cursive::Cursive,
    glues_core::{
        data::Directory,
        state::notebook::{DirectoryItem, NotebookState},
        NotebookEvent,
    },
};

pub fn add_directory(siv: &mut Cursive, directory: Directory) {
    if !siv
        .state::<NotebookState>()
        .check_opened(&directory.parent_id)
    {
        siv.dispatch(NotebookEvent::OpenDirectory(directory.parent_id.clone()));
    } else {
        let mut container = Node::notebook()
            .note_tree()
            .directory(&directory.parent_id)
            .note_list()
            .find(siv);

        let directory_item = DirectoryItem {
            directory: directory.clone(),
            children: None,
        };
        container.add_child(render_directory(siv, directory_item));
    }

    siv.focus_on_next_tick(
        Node::notebook()
            .note_tree()
            .directory(&directory.id)
            .name_button()
            .name(),
    );
}
