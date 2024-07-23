use {
    crate::{traits::*, views::editor::content::render_content, Node},
    cursive::{views::LayerPosition, Cursive},
    glues_core::data::Note,
};

pub fn open_note(siv: &mut Cursive, note: Note, content: String) {
    Node::note_tree()
        .note(&note.id)
        .name_button()
        .find(siv)
        .disable();

    let content = render_content(siv, note.name, content);
    let mut editor = Node::editor().find(siv);
    editor.add_fullscreen_layer(content);
    editor.remove_layer(LayerPosition::FromBack(0));

    siv.cb_sink()
        .send(Box::new(move |siv| {
            siv.focus_name(&Node::editor().name_button().name())
                .log_unwrap();
        }))
        .log_unwrap();
}
