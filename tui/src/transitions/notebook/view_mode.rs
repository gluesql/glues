use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::{data::Note, NotebookEvent},
};

pub fn view_mode(siv: &mut Cursive, _note: Note) {
    Node::notebook().editor().panel().find(siv).set_title("");
    let mut content = Node::notebook().editor().content().find(siv);
    content.disable();

    siv.focus_name(&Node::notebook().editor().name_button().name())
        .log_unwrap();

    siv.dispatch(NotebookEvent::UpdateNoteContent(
        content.get_content().to_owned(),
    ))
}
