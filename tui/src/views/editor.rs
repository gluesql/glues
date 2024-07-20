pub mod content;

use {
    crate::{traits::*, Node},
    content::render_content,
    cursive::{
        align::HAlign,
        view::Nameable,
        views::{NamedView, PaddedView, Panel, StackView},
        Cursive,
    },
};

pub fn render_editor(siv: &mut Cursive) -> NamedView<Panel<PaddedView<NamedView<StackView>>>> {
    let content = render_content(siv, "hello :)".to_owned());
    let stack = StackView::new()
        .fullscreen_layer(content)
        .with_name(Node::editor().name());

    let padded_view = PaddedView::lrtb(1, 1, 0, 0, stack);
    Panel::new(padded_view)
        .title(":D")
        .title_position(HAlign::Left)
        .with_name(Node::editor().panel().name())
}
