use {
    crate::views::notes::{editor::render_editor, note_tree::render_note_tree},
    cursive::{view::View, views::LinearLayout, Cursive},
};

pub fn render_note_sheet(siv: &mut Cursive) -> impl View {
    LinearLayout::horizontal()
        .child(render_note_tree(siv))
        .child(render_editor(siv))
}
