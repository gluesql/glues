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
    let description = TextView::new(description).with_name(statusbar_node.description().name());

    let shortcuts = siv.glues().state.shortcuts().join(", ");
    let shortcuts = TextView::new(shortcuts)
        .with_name(statusbar_node.shortcuts().name())
        .full_width();

    let layout = LinearLayout::horizontal()
        .child(shortcuts)
        .child(description);
    let padded_view = PaddedView::lrtb(1, 2, 0, 0, layout);

    Layer::new(padded_view).full_width()
}
