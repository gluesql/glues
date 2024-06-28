pub mod directory;
pub mod note;

use {
    crate::{node::Node, traits::*},
    cursive::{
        view::{Nameable, Resizable},
        views::{PaddedView, Panel},
        Cursive, View,
    },
    directory::render_directory,
    glues_core::state::{note_tree::NoteTreeState, State},
};

pub fn render_note_tree(siv: &mut Cursive) -> impl View {
    let NoteTreeState { root, .. } = match &siv.glues().state {
        State::NoteTree(state) => state,
        state => {
            let msg = format!("state not allowed: {}", state.describe());

            log(&msg);
            panic!("{msg}");
        }
    };

    let directory_item = root.clone();
    let layout = render_directory(siv, directory_item)
        .with_name(Node::note_tree().note_list().name())
        .min_width(40);
    let padded_view = PaddedView::lrtb(1, 1, 0, 1, layout);

    Panel::new(padded_view).full_height()
}
