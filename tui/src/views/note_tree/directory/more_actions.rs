use {
    crate::{actions, traits::*},
    cursive::{
        align::HAlign,
        views::{Button, CircularFocus, Dialog, DummyView, LinearLayout, TextView},
        Cursive, With,
    },
    glues_core::data::Directory,
    std::rc::Rc,
};

pub fn render_more_actions(directory: Directory) -> CircularFocus<Dialog> {
    let directory = Rc::new(directory);
    let label = TextView::new(format!("'{}'", &directory.name)).h_align(HAlign::Center);
    let add_note_button = Button::new("Add Note", on_add_note_click(Rc::clone(&directory)));
    let rename_button = Button::new("Rename", on_rename_click(directory));
    let cancel_button = Button::new("Cancel", |siv| {
        siv.pop_layer();
    });

    let actions = LinearLayout::vertical()
        .child(label)
        .child(DummyView)
        .child(add_note_button)
        .child(rename_button)
        .child(DummyView)
        .child(cancel_button);

    Dialog::new()
        .title("More Actions")
        .content(actions)
        .padding_lrtb(3, 3, 1, 1)
        .wrap_with(CircularFocus::new)
        .wrap_tab()
}

fn on_rename_click(directory: Rc<Directory>) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv: &mut Cursive| {
        let directory = Rc::clone(&directory);
        let message = "New name?";

        siv.pop_layer();
        siv.prompt(message, move |siv, directory_name| {
            actions::rename_directory(siv, &directory, directory_name);
        });
    }
}

fn on_add_note_click(directory: Rc<Directory>) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv: &mut Cursive| {
        let directory = Rc::clone(&directory);
        let message = "Note name?";

        siv.pop_layer();
        siv.prompt(message, move |siv, note_name| {
            actions::add_note(siv, &directory.id, note_name);
        });
    }
}
