use {
    crate::CursiveExt,
    cursive::{
        views::{Dialog, TextView},
        Cursive,
    },
    glues_core::Event,
};

pub fn render_confirm<F>(message: &str, on_confirm: F) -> Dialog
where
    F: Fn(&mut Cursive) + 'static,
{
    Dialog::around(TextView::new(message))
        .title("Confirm")
        .button("OK", move |siv| {
            siv.pop_layer();
            on_confirm(siv);
        })
        .button("Cancel", |siv| {
            siv.dispatch2(Event::Cancel);
            siv.pop_layer();
        })
        .padding_lrtb(3, 3, 1, 1)
}
