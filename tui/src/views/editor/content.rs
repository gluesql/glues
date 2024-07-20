use {
    crate::{traits::*, Node},
    cursive::{
        view::{Nameable, Resizable},
        views::{LinearLayout, TextArea},
        Cursive, View,
    },
};

pub fn render_content(_siv: &mut Cursive, content: String) -> impl View {
    let textarea = TextArea::new()
        .content(content)
        .disabled()
        .with_name(Node::editor().content().name())
        .full_screen();

    LinearLayout::vertical().child(textarea)
}
