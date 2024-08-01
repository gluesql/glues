use {
    crate::{traits::*, views::notebook::editor::content::render_content, Node},
    cursive::{views::LayerPosition, Cursive},
    glues_core::data::Note,
};

pub fn open_note(siv: &mut Cursive, note: Note, content: String) {
    Node::notes()
        .note_tree()
        .note(&note.id)
        .name_button()
        .find(siv)
        .disable();

    let content = render_content(siv, note.name, content);
    let mut editor = Node::notes().editor().find(siv);
    editor.add_fullscreen_layer(content);
    editor.remove_layer(LayerPosition::FromBack(0));

    siv.focus_on_next_tick(Node::notes().editor().name_button().name());
}
