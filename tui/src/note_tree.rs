mod directory;
mod note;

use {
    directory::directory,
    note::note,
    cursive::{
        theme::{BaseColor, ColorStyle},
        views::{LinearLayout, PaddedView, Panel, TextView},
        view::Resizable,
        View,
        Cursive,
    },
    glues_core::{Glues, data::{Directory}},
};

pub async fn note_tree(siv: &mut Cursive) -> impl View {
    let text_view = TextView::new("note tree").style(ColorStyle::back(BaseColor::Yellow));
    
    let mut glues: &mut Glues = siv.user_data().unwrap();
    let notes = glues.fetch_notes(glues.root_id.clone()).await;
    let _directories: Vec<Directory> = glues.fetch_directories(glues.root_id.clone()).await;
    
    let depth = 0;
    let mut layout = LinearLayout::vertical()
        .child(text_view)
        .child(directory(siv, depth, "dir 1".to_owned()));
        /*
        .child(note(glues, depth, "note 1"))
        .child(note(glues, depth, "note 2"));
        */

    for data in notes {
        layout.add_child(note(siv, depth, data).await);
    }
    
    let layout = layout.min_width(25);

    let padded_view = PaddedView::lrtb(1, 1, 0, 1, layout);

    Panel::new(padded_view)
}
