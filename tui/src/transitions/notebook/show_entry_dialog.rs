use {
    crate::{traits::*, views::entry::render_entry},
    cursive::{views::Dialog, Cursive},
    glues_core::NotebookEvent,
};

pub fn show_entry_dialog(siv: &mut Cursive) {
    let dialog = Dialog::around(render_entry())
        .title("Glues")
        .button("Close", move |siv| {
            siv.pop_layer();
            siv.dispatch(NotebookEvent::CloseEntryDialog);
        });

    siv.add_layer(dialog);
}
