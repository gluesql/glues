use {
    crate::{logger::log, traits::*, Node},
    cursive::Cursive,
};

pub fn update_statusbar(siv: &mut Cursive) {
    let statusbar_node = Node::statusbar();

    let description = siv.glues().state.describe();
    statusbar_node
        .description()
        .find(siv)
        .set_content(&description);

    let shortcuts = siv.glues().state.shortcuts().join(", ");
    statusbar_node.shortcuts().find(siv).set_content(&shortcuts);

    log(&format!("[state] {description} / {shortcuts}"));
}
