use {
    crate::traits::*,
    cursive::{
        align::HAlign,
        views::{Button, CircularFocus, Dialog, DummyView, LinearLayout, TextView},
        Cursive, With,
    },
    glues_core::{data::Note, Event},
    std::rc::Rc,
};

pub fn render_more_actions(note: Note) -> CircularFocus<Dialog> {
    let label = TextView::new(format!("'{}'", &note.name)).h_align(HAlign::Center);
    let remove_button = Button::new("Remove", on_remove_click(note.clone()));
    let rename_button = Button::new("Rename", |siv| {
        let message = "New name?";

        siv.pop_layer();
        siv.prompt(message, move |siv, note_name| {
            siv.dispatch2(Event::RenameNote(note_name.to_owned()));
        });
    });
    let cancel_button = Button::new("Cancel", |siv| {
        siv.dispatch2(Event::CloseNoteActionsDialog);
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
        .wrap_with(CircularFocus::new)
        .wrap_tab()
}

fn on_remove_click(note: Note) -> impl for<'a> Fn(&'a mut Cursive) {
    let note = Rc::new(note);

    move |siv: &mut Cursive| {
        let note = Rc::clone(&note);
        let message = format!("Removes '{}'", &note.name);

        siv.pop_layer();
        siv.confirm(message, move |siv| {
            siv.dispatch2(Event::RemoveNote);
        });
    }
}
