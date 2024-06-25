use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
};

pub fn update_statusbar(siv: &mut Cursive) {
    let description = siv.glues().state.describe();
    Node::statusbar()
        .description()
        .find(siv)
        .set_content(&description);

    log(&format!("[state] {description}"));
}
