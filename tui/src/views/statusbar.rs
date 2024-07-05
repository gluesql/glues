use {
    crate::{traits::*, Node},
    cursive::{
        view::{Nameable, Resizable},
        views::{Layer, LinearLayout, PaddedView, TextView},
        Cursive, View,
    },
};

pub fn render_statusbar(siv: &mut Cursive) -> impl View {
    let statusbar_node = Node::statusbar();

    let description = siv.glues().state.describe();
    let description = TextView::new(description)
        .with_name(statusbar_node.description().name())
        .full_width();

    let shortcuts = siv.glues().state.shortcuts().join(", ");
    let shortcuts = TextView::new(shortcuts).with_name(statusbar_node.shortcuts().name());

    let layout = LinearLayout::horizontal()
        .child(description)
        .child(shortcuts);
    let padded_view = PaddedView::lrtb(2, 2, 0, 0, layout);

    Layer::new(padded_view).full_width()
}
