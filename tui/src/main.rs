mod actions;
mod components;
mod cursive_ext;
mod logger;
mod node;
mod views;

mod traits {
    pub(crate) use crate::{
        cursive_ext::CursiveExt,
        logger::*,
        node::{ViewFinder, ViewNamed},
    };
}

use {
    cursive::{
        view::Resizable,
        views::{CircularFocus, LinearLayout, PaddedView, StackView},
        Cursive, With,
    },
    futures::executor::block_on,
    glues_core::Glues,
    logger::log,
    node::Node,
    traits::*,
    views::{
        editor::editor, menubar::menubar, note_tree::render_note_tree, statusbar::render_statusbar,
    },
};

fn main() {
    cursive::logger::init();
    logger::init();

    log("logger initialized");

    let mut glues = block_on(Glues::new());

    let directory_id = glues
        .add_directory(glues.root_id.clone(), "Directory 01".to_owned())
        .log_unwrap()
        .id;

    let sample_notes = [
        ("Sample 001", glues.root_id.clone()),
        ("Note for the note", glues.root_id.clone()),
        ("Glocery items", directory_id.clone()),
        ("Sub item note sample", directory_id.clone()),
        ("Hello Glues!", directory_id.clone()),
    ];

    for (name, directory_id) in sample_notes {
        glues.add_note(directory_id, name.to_owned()).log_unwrap();
    }

    log("added sample notes & directories");

    let mut siv = cursive::default();
    siv.set_user_data(glues);
    siv.add_global_callback('a', Cursive::toggle_debug_console);

    let layout = LinearLayout::horizontal()
        .child(render_note_tree(&mut siv))
        .child(editor(&mut siv))
        .wrap_with(CircularFocus::new);
    let mut stack_view = StackView::new();
    stack_view.add_layer(layout);
    let padded_view = PaddedView::lrtb(0, 1, 0, 1, stack_view);

    let statusbar = render_statusbar(&mut siv);
    let layout = LinearLayout::vertical()
        .child(padded_view)
        .child(statusbar)
        .full_screen();

    siv.screen_mut().add_transparent_layer(layout);

    menubar(&mut siv);
    siv.run();
}
