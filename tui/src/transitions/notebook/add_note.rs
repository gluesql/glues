use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::{data::Note, NotebookEvent},
};

pub fn add_note(siv: &mut Cursive, note: Note) {
    siv.dispatch(NotebookEvent::OpenDirectory(note.directory_id.clone()));
    siv.focus_on_next_tick(
        Node::notebook()
            .note_tree()
            .note(&note.id)
            .name_button()
            .name(),
    );
}
