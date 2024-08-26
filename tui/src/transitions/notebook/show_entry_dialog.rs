use {
    crate::{traits::*, views::entry::render_entry, wrapper::JkWrapper},
    cursive::{views::Dialog, Cursive, With},
    glues_core::NotebookEvent,
};

pub fn show_entry_dialog(siv: &mut Cursive) {
    let dialog = Dialog::around(render_entry())
        .title("Glues")
        .button("Close", move |siv| {
            siv.pop_layer();
            siv.dispatch(NotebookEvent::CloseEntryDialog);
        })
        .wrap_with(JkWrapper::new);

    siv.add_layer(dialog);
}
