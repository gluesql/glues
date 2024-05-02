use cursive::{
    views::{CircularFocus, LinearLayout},
    With,
};
use futures::executor::block_on;
use glues_core::Glues;

mod editor;
mod note_tree;

use editor::editor;
use note_tree::note_tree;

fn main() {
    block_on(run());
}

async fn run() {
    cursive::logger::init();

    let mut glues = Glues::new().await;

    glues
        .add_note(glues.root_id.clone(), "Test Note 1".to_owned())
        .await;
    glues
        .add_note(glues.root_id.clone(), "Test Note 2".to_owned())
        .await;
    glues
        .add_directory(glues.root_id.clone(), "test directory 11".to_owned())
        .await;

    let notes = glues.fetch_notes(glues.root_id.clone()).await;
    println!("notes: {notes:?}");

    let directories = glues.fetch_directories(glues.root_id.clone()).await;
    println!("directories: {directories:?}");

    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();
    siv.set_user_data(glues);

    // Creates a dialog with a single "Quit" button
    /*
    siv.add_layer(
        Dialog::around(TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );
    */

    let layout = LinearLayout::horizontal()
        .child(note_tree(&mut siv).await)
        .child(editor(&mut siv))
        .wrap_with(CircularFocus::new);

    siv.add_fullscreen_layer(layout);

    // let mut v_split = LinearLayout::new(Orientation::Vertical);
    /*
    v_split.add_child(
    );
    */

    // Starts the event loop.
    siv.run();
}
