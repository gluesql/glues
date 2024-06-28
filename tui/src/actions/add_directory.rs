use {
    crate::{actions, traits::*, views::note_tree::directory::render_directory, Node},
    cursive::Cursive,
    glues_core::{state::note_tree::DirectoryItem, state::State, types::DirectoryId},
};

pub fn add_directory(siv: &mut Cursive, parent_id: &DirectoryId, directory_name: &str) {
    // data
    let directory = siv
        .glues()
        .db
        .add_directory(parent_id.clone(), directory_name.to_owned())
        .log_unwrap();
    let directory_id = directory.id.clone();

    let opened = match &siv.glues().state {
        State::NoteTree(state) => state.check_opened(parent_id),
        _ => panic!(),
    };

    // ui
    // if !siv.glues().db.check_opened(parent_id) {
    if opened {
        actions::open_directory(siv, parent_id);
    } else {
        let mut container = if &siv.glues().root_id == parent_id {
            Node::note_tree().note_list().find(siv)
        } else {
            Node::note_tree().directory(parent_id).note_list().find(siv)
        };

        let directory_item = DirectoryItem {
            directory,
            children: None,
        };
        container.add_child(render_directory(siv, directory_item));
    }

    siv.cb_sink()
        .send(Box::new(move |siv| {
            siv.focus_name(
                &Node::note_tree()
                    .directory(&directory_id)
                    .name_button()
                    .name(),
            )
            .log_unwrap();
        }))
        .log_unwrap();
}
