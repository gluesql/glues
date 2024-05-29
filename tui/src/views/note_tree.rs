mod directory;
mod note;

use {
    crate::{cursive_ext::CursiveExt, logger::*},
    cursive::{
        align::HAlign,
        view::Resizable,
        views::{LinearLayout, PaddedView, Panel},
        Cursive, View,
    },
    directory::render_directory,
    note::render_note,
};

pub fn render_note_tree(siv: &mut Cursive) -> impl View {
    let root_id = siv.glues().root_id.clone();
    let notes = siv.glues().fetch_notes(root_id.clone()).log_unwrap();
    let directories = siv.glues().fetch_directories(root_id.clone()).log_unwrap();

    let mut layout = LinearLayout::vertical();

    for data in directories {
        layout.add_child(render_directory(siv, data));
    }

    for data in notes {
        layout.add_child(render_note(data));
    }

    let layout = layout.min_width(40);
    let padded_view = PaddedView::lrtb(1, 1, 0, 1, layout);

    Panel::new(padded_view)
        .title("Notes")
        .title_position(HAlign::Left)
}
