use {
    super::render_directory,
    crate::views::notebook::note_tree::note::render_note,
    cursive::{
        views::{LinearLayout, PaddedView},
        Cursive,
    },
    glues_core::{data::Note, state::notebook::DirectoryItem},
};

pub fn render_item_list(
    siv: &mut Cursive,
    container: &mut LinearLayout,
    directories: Vec<DirectoryItem>,
    notes: Vec<Note>,
) {
    for directory_item in directories {
        let view = render_directory(siv, directory_item);

        container.add_child(PaddedView::lrtb(2, 0, 0, 0, view));
    }

    for child in notes {
        container.add_child(PaddedView::lrtb(2, 0, 0, 0, render_note(child)));
    }
}
