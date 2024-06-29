use {
    crate::{actions, traits::*, views::note_tree::note::render_note, Node},
    cursive::Cursive,
    glues_core::{state::note_tree::NoteTreeState, types::DirectoryId},
};

pub fn add_note(siv: &mut Cursive, directory_id: &DirectoryId, note_name: &str) {
    // data
    let note = siv
        .glues()
        .db
        .add_note(directory_id.clone(), note_name.to_owned())
        .log_unwrap();
    let note_id = note.id.clone();

    // ui
    if siv.state::<NoteTreeState>().check_opened(directory_id) {
        actions::open_directory(siv, directory_id);
    } else {
        let mut container = if &siv.glues().root_id == directory_id {
            Node::note_tree().note_list().find(siv)
        } else {
            Node::note_tree()
                .directory(directory_id)
                .note_list()
                .find(siv)
        };

        let note_view = render_note(note);
        container.add_child(note_view);
    }

    siv.cb_sink()
        .send(Box::new(move |siv| {
            siv.focus_name(&Node::note_tree().note(&note_id).name_button().name())
                .log_unwrap();
        }))
        .log_unwrap();
}
