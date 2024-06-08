use cursive::{event::Key, Cursive};

pub fn menubar(siv: &mut Cursive) {
    siv.menubar()
        .add_leaf("Quit", |s| s.quit())
        .add_delimiter()
        .add_leaf("[ESC] to focus", |_| {});

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.set_autohide_menu(false);
}
