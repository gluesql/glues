use {
    crate::{traits::*, Node},
    cursive::Cursive,
};

pub fn edit_mode(siv: &mut Cursive) {
    Node::notebook().editor().panel().find(siv).set_title("*");
    Node::notebook().editor().content().find(siv).enable();

    siv.focus_name(&Node::notebook().editor().content().name())
        .log_unwrap();
}
