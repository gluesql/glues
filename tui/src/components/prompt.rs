use {
    crate::CursiveExt,
    cursive::{
        view::{Nameable, Resizable},
        views::{Dialog, EditView, LinearLayout, TextView},
        Cursive,
    },
};

const EDIT_VIEW_NAME: &str = "edit_view_name";

pub fn render_prompt<F>(message: &str, on_submit: F) -> Dialog
where
    F: Fn(&mut Cursive, &str) + Clone + 'static,
{
    let content = LinearLayout::vertical()
        .child(TextView::new(message))
        .child({
            let on_submit = on_submit.clone();

            EditView::new()
                .on_submit(move |siv: &mut Cursive, value| {
                    siv.pop_layer();
                    on_submit(siv, value);
                })
                .with_name(EDIT_VIEW_NAME)
                .fixed_width(20)
        });

    Dialog::around(content)
        .title("Prompt")
        .button("OK", move |siv| {
            let value = siv.find::<EditView>(EDIT_VIEW_NAME).get_content();
            on_submit(siv, &value);

            siv.pop_layer();
        })
        .button("Cancel", |siv| {
            siv.pop_layer();
        })
        .padding_lrtb(3, 3, 1, 1)
}
