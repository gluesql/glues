use {
    crate::{traits::*, Node},
    cursive::{
        view::{Nameable, Resizable},
        views::{Button, DummyView, LinearLayout, TextArea, TextView},
        Cursive, View,
    },
    glues_core::Event,
};

pub fn render_content(_siv: &mut Cursive, name: String, content: String) -> impl View {
    let name_button = Button::new_raw(name, |siv| {
        siv.dispatch(Event::EditNote);
    })
    .with_name(Node::notebook().editor().name_button().name());
    let name_row = LinearLayout::horizontal()
        .child(TextView::new("Name: "))
        .child(name_button);
    let textarea = TextArea::new()
        .content(content)
        .disabled()
        .with_name(Node::notebook().editor().content().name())
        .full_screen();
    let status = LinearLayout::horizontal()
        .child(DummyView.full_width())
        .child(TextView::new("loaded").with_name(Node::notebook().editor().status().name()));

    LinearLayout::vertical()
        .child(name_row)
        .child(DummyView)
        .child(textarea)
        .child(status)
}
