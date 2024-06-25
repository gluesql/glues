use {
    crate::actions,
    cursive::{
        event::Key,
        menu::{Item, Tree},
        Cursive,
    },
};

pub fn menubar(siv: &mut Cursive) {
    siv.menubar()
        .add_subtree(
            "Glues",
            Tree::new()
                .leaf("New Notes", |siv| {
                    actions::initialize(siv);
                })
                .leaf("Quit", |siv| siv.quit()),
        )
        .add_delimiter()
        .item(Item::leaf("[ESC] to focus", |_| {}).disabled());

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.set_autohide_menu(false);
}
