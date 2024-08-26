pub mod directory;
pub mod note;

use {
    crate::{node::Node, traits::*},
    cursive::{
        view::{Nameable, Resizable, Scrollable},
        views::{PaddedView, Panel},
        Cursive, View,
    },
    directory::render_directory,
    glues_core::state::notebook::NotebookState,
};

pub fn render_note_tree(siv: &mut Cursive) -> impl View {
    let NotebookState { root, .. } = siv.state();
    let directory_item = root.clone();
    let layout = render_directory(siv, directory_item)
        .with_name(Node::notebook().note_tree().note_list().name())
        .scrollable()
        .min_width(40);
    let padded_view = PaddedView::lrtb(1, 1, 0, 1, layout);

    Panel::new(padded_view).full_height()
}
