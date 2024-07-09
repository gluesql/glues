use {
    crate::{traits::*, views::note_tree::note::render_note, Node},
    cursive::Cursive,
    glues_core::{data::Note, state::note_tree::NoteTreeState, Event},
};

pub fn add_note(siv: &mut Cursive, note: Note) {
    if !siv
        .state::<NoteTreeState>()
        .check_opened(&note.directory_id)
    {
        siv.dispatch2(Event::OpenDirectory(note.directory_id.clone()));
    } else {
        let mut container = Node::note_tree()
            .directory(&note.directory_id)
            .note_list()
            .find(siv);
        let note_view = render_note(note.clone());
        container.add_child(note_view);
    }

    siv.cb_sink()
        .send(Box::new(move |siv| {
            siv.focus_name(&Node::note_tree().note(&note.id).name_button().name())
                .log_unwrap();
        }))
        .log_unwrap();
}
