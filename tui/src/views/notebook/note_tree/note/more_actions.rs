use {
    crate::{traits::*, wrapper::JkWrapper},
    cursive::{
        align::HAlign,
        view::View,
        views::{Button, Dialog, DummyView, LinearLayout, TextView},
        Cursive, With,
    },
    glues_core::{data::Note, NotebookEvent},
    std::sync::Arc,
};

pub fn render_more_actions(note: Note) -> impl View {
    let label = TextView::new(format!("'{}'", &note.name)).h_align(HAlign::Center);
    let remove_button = Button::new("Remove", on_remove_click(note.clone()));
    let rename_button = Button::new("Rename", |siv| {
        let message = "New name?";

        siv.pop_layer();
        siv.prompt(message, move |siv, note_name| {
            siv.dispatch(NotebookEvent::RenameNote(note_name.to_owned()));
        });
    });
    let cancel_button = Button::new("Cancel", |siv| {
        siv.dispatch(NotebookEvent::CloseNoteActionsDialog);
        siv.pop_layer();
    });

    let actions = LinearLayout::vertical()
        .child(label)
        .child(DummyView)
        .child(rename_button)
        .child(remove_button)
        .child(DummyView)
        .child(cancel_button);

    Dialog::new()
        .title("More Actions")
        .content(actions)
        .padding_lrtb(3, 3, 1, 1)
        .wrap_with(JkWrapper::new)
}

fn on_remove_click(note: Note) -> impl for<'a> Fn(&'a mut Cursive) {
    let note = Arc::new(note);

    move |siv: &mut Cursive| {
        let note = Arc::clone(&note);
        let message = format!("Removes '{}'", &note.name);

        siv.pop_layer();
        siv.confirm(message, move |siv| {
            siv.dispatch(NotebookEvent::RemoveNote);
        });
    }
}
