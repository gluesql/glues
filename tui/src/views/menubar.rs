use {
    crate::traits::*,
    cursive::{
        menu::{Item, Tree},
        Cursive,
    },
    glues_core::EntryEvent,
};

pub fn menubar(siv: &mut Cursive) {
    siv.menubar()
        .add_subtree(
            "Glues",
            Tree::new()
                .leaf("New Notes", |siv| {
                    siv.dispatch(EntryEvent::Initialize);
                })
                .leaf("Quit", |siv| siv.quit()),
        )
        .add_delimiter()
        .item(Item::leaf("[ESC] to focus", |_| {}).disabled());

    siv.set_autohide_menu(false);
}
