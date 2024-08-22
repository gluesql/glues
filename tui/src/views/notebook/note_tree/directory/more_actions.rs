use {
    crate::traits::*,
    cursive::{
        align::HAlign,
        views::{Button, CircularFocus, Dialog, DummyView, LinearLayout, TextView},
        Cursive, With,
    },
    glues_core::{data::Directory, NotebookEvent},
    std::sync::Arc,
};

pub fn render_more_actions(directory: Directory) -> CircularFocus<Dialog> {
    let directory = Arc::new(directory);
    let label = TextView::new(format!("'{}'", &directory.name)).h_align(HAlign::Center);
    let add_note_button = Button::new("Add Note", on_add_note_click);
    let add_directory_button = Button::new("Add Directory", on_add_directory_click);
    let rename_button = Button::new("Rename", |siv| {
        let message = "New name?";

        siv.pop_layer();
        siv.prompt(message, |siv, directory_name| {
            siv.dispatch(NotebookEvent::RenameDirectory(directory_name.to_owned()));
        });
    });
    let remove_button = Button::new("Remove", on_remove_click(directory));
    let cancel_button = Button::new("Cancel", |siv| {
        siv.dispatch(NotebookEvent::CloseDirectoryActionsDialog);
        siv.pop_layer();
    });

    let actions = LinearLayout::vertical()
        .child(label)
        .child(DummyView)
        .child(add_note_button)
        .child(add_directory_button)
        .child(rename_button)
        .child(remove_button)
        .child(DummyView)
        .child(cancel_button);

    Dialog::new()
        .title("More Actions")
        .content(actions)
        .padding_lrtb(3, 3, 1, 1)
        .wrap_with(CircularFocus::new)
        .wrap_tab()
}

fn on_remove_click(directory: Arc<Directory>) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv: &mut Cursive| {
        let directory = Arc::clone(&directory);
        let message = format!("Removes {}", directory.name);

        siv.pop_layer();
        siv.confirm(message, move |siv| {
            siv.dispatch(NotebookEvent::RemoveDirectory)
        });
    }
}

fn on_add_note_click(siv: &mut Cursive) {
    let message = "Note name?";

    siv.pop_layer();
    siv.prompt(message, move |siv, note_name| {
        siv.dispatch(NotebookEvent::AddNote(note_name.to_owned()));
    });
}

fn on_add_directory_click(siv: &mut Cursive) {
    let message = "Directory name?";

    siv.pop_layer();
    siv.prompt(message, move |siv, directory_name| {
        siv.dispatch(NotebookEvent::AddDirectory(directory_name.to_owned()));
    });
}
