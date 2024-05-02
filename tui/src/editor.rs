use cursive::{
    view::{Nameable, Resizable},
    views::{PaddedView, TextView},
    View,
    Cursive,
};

pub fn editor(siv: &mut Cursive) -> impl View {
    PaddedView::lrtb(
        1,
        1,
        1,
        1,
        TextView::new("editor").with_name("temp_text").full_screen(),
    )
}
