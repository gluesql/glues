use {
    crate::{traits::*, views::notes::note_tree::note::render_note, Node},
    cursive::Cursive,
    glues_core::{data::Note, state::notes::NotesState, Event},
};

pub fn add_note(siv: &mut Cursive, note: Note) {
    if !siv.state::<NotesState>().check_opened(&note.directory_id) {
        siv.dispatch(Event::OpenDirectory(note.directory_id.clone()));
    } else {
        let mut container = Node::note_tree()
            .directory(&note.directory_id)
            .note_list()
            .find(siv);
        let note_view = render_note(note.clone());
        container.add_child(note_view);
    }

    siv.focus_on_next_tick(Node::note_tree().note(&note.id).name_button().name());
}
