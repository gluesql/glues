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
                .leaf("New Notes - Memory", |siv| {
                    siv.dispatch(EntryEvent::OpenMemory);
                })
                .leaf("New Notes - CSV", |siv| {
                    siv.prompt("Path?", |siv, path| {
                        siv.dispatch(EntryEvent::OpenCsv(path.to_owned()));
                    })
                })
                .leaf("New Notes - JSON", |siv| {
                    siv.prompt("Path?", |siv, path| {
                        siv.dispatch(EntryEvent::OpenJson(path.to_owned()));
                    })
                })
                .leaf("Quit", |siv| siv.quit()),
        )
        .add_delimiter()
        .item(Item::leaf("[ESC] to focus", |_| {}).disabled());

    siv.set_autohide_menu(false);
}
