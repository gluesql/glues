use cursive::{
    views::{Dialog, TextView},
    Cursive,
};

pub fn render_alert<F>(message: &str, on_close: F) -> Dialog
where
    F: Fn(&mut Cursive) + Send + Sync + 'static,
{
    Dialog::around(TextView::new(message))
        .title("Alert")
        .button("OK", move |siv| {
            siv.pop_layer();
            on_close(siv);
        })
        .padding_lrtb(3, 3, 1, 1)
}
