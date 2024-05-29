mod components;
mod cursive_ext;
mod logger;
mod views;

use {
    cursive::{
        views::{CircularFocus, LinearLayout},
        With,
    },
    futures::executor::block_on,
    glues_core::Glues,
    logger::*,
    views::{editor::editor, note_tree::render_note_tree},
};

fn main() {
    cursive::logger::init();
    logger::init();

    log("logger initialized");

    let mut glues = block_on(Glues::new());

    let directory_id = glues
        .add_directory(glues.root_id.clone(), "Directory 01".to_owned())
        .log_unwrap();

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

    let layout = LinearLayout::horizontal()
        .child(render_note_tree(&mut siv))
        .child(editor(&mut siv))
        .wrap_with(CircularFocus::new);
    siv.add_fullscreen_layer(layout);

    siv.run();
}
