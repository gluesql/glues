use cursive::{event::Key, menu::Item as MenuItem, Cursive};

pub fn menubar(siv: &mut Cursive) {
    siv.menubar()
        .add_leaf("Quit", |s| s.quit())
        .add_delimiter()
        .item(MenuItem::leaf("[ESC] to focus", |_| {}).disabled());

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.set_autohide_menu(false);
}
