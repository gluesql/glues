use {
    crate::{traits::*, Node},
    cursive::Cursive,
};

pub fn edit_mode(siv: &mut Cursive) {
    Node::notes().editor().panel().find(siv).set_title("*");
    Node::notes().editor().content().find(siv).enable();

    siv.focus_name(&Node::notes().editor().content().name())
        .log_unwrap();
}
