use {
    crate::views::{editor::editor, note_tree::render_note_tree},
    cursive::{
        view::View,
        views::{CircularFocus, LinearLayout},
        Cursive, With,
    },
};

pub fn render_note_sheet(siv: &mut Cursive) -> impl View {
    LinearLayout::horizontal()
        .child(render_note_tree(siv))
        .child(editor(siv))
        .wrap_with(CircularFocus::new)
}
