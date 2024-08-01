use {
    super::render_directory,
    crate::{traits::*, views::notebook::note_tree::note::render_note, Node},
    cursive::{
        view::Nameable,
        views::{LinearLayout, PaddedView},
        Cursive, View,
    },
    glues_core::{data::Note, state::notebook::DirectoryItem, types::DirectoryId},
};

pub fn render_item_list(
    siv: &mut Cursive,
    directory_id: DirectoryId,
    directories: Vec<DirectoryItem>,
    notes: Vec<Note>,
) -> impl View {
    let mut layout = LinearLayout::vertical();

    for directory_item in directories {
        layout.add_child(render_directory(siv, directory_item));
    }

    for child in notes {
        layout.add_child(render_note(child));
    }

    let layout = layout.with_name(
        Node::notebook()
            .note_tree()
            .directory(&directory_id)
            .note_list()
            .name(),
    );

    PaddedView::lrtb(2, 0, 0, 0, layout)
}
