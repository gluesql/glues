use {
    crate::cursive_ext::CursiveExt,
    async_io::block_on,
    cursive::{
        align::HAlign,
        views::{Button, CircularFocus, Dialog, DummyView, LinearLayout, TextView},
        Cursive, With,
    },
    glues_core::{data::Note, types::NoteId},
};

pub fn render_more_actions(note: Note) -> CircularFocus<Dialog> {
    let label = TextView::new(format!("'{}'", &note.name)).h_align(HAlign::Center);
    let remove_button = Button::new("Remove", on_remove_click(note));
    let rename_button = Button::new("Rename", |_siv| {});
    let cancel_button = Button::new("Cancel", |siv| {
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
    let callback = move |siv: &mut Cursive| {
        let message = format!("Removes '{}'", &note.name);

        siv.pop_layer();
        siv.confirm(message, on_confirm(note.id.clone()));
    };

    fn on_confirm(note_id: NoteId) -> impl for<'a> Fn(&'a mut Cursive) {
        move |siv| {
            let note_id = note_id.clone();

            let wow = block_on(siv.glues().remove_note(note_id));

            siv.alert(wow, |_| {});
        }
    }

    callback
}
