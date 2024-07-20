use {
    crate::{traits::*, views::editor::content::render_content, Node},
    cursive::{views::LayerPosition, Cursive},
    glues_core::data::Note,
};

pub fn open_note(siv: &mut Cursive, note: Note, content: String) {
    Node::editor().panel().find(siv).set_title(note.name);

    let content = render_content(siv, content);
    let mut editor = Node::editor().find(siv);
    editor.add_fullscreen_layer(content);
    editor.remove_layer(LayerPosition::FromBack(0));
}
