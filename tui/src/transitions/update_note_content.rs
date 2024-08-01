use {
    crate::{traits::*, Node},
    cursive::Cursive,
};

pub fn update_note_content(siv: &mut Cursive) {
    Node::notes()
        .editor()
        .status()
        .find(siv)
        .set_content("saved");
}
