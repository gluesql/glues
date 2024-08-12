mod components;
mod cursive_ext;
#[macro_use]
mod logger;
mod node;
mod transitions;
mod views;

mod traits {
    pub(crate) use crate::{
        cursive_ext::CursiveExt,
        logger::*,
        node::{ViewFinder, ViewNamed},
    };
}

use {
    cursive::{
        event::{Event, Key},
        view::{Nameable, Resizable},
        views::{DummyView, LinearLayout, PaddedView, Panel, StackView},
        Cursive,
    },
    futures::executor::block_on,
    glues_core::{Glues, KeyEvent},
    node::Node,
    traits::*,
    transitions::handle_event,
    views::{entry::render_entry, statusbar::render_statusbar},
};

fn main() {
    cursive::logger::init();
    logger::init();

    log!("logger initialized");

    let glues = block_on(Glues::new());

    let mut siv = cursive::default();
    siv.set_user_data(glues);
    siv.add_global_callback('a', Cursive::toggle_debug_console);

    for (key, event) in [
        (Event::Char('b'), KeyEvent::B),
        (Event::Char('e'), KeyEvent::E),
        (Event::Char('h'), KeyEvent::H),
        (Event::Char('j'), KeyEvent::J),
        (Event::Char('k'), KeyEvent::K),
        (Event::Char('l'), KeyEvent::L),
        (Event::Char('m'), KeyEvent::M),
        (Event::Char('o'), KeyEvent::O),
        (Key::Left.into(), KeyEvent::Left),
        (Key::Right.into(), KeyEvent::Right),
        (Key::Esc.into(), KeyEvent::Esc),
    ] {
        siv.set_global_callback(key, move |siv| {
            handle_event(siv, event.into());
        });
    }

    let entry_view = Panel::new(render_entry()).title("Glues");
    let stack_view = StackView::new()
        .transparent_layer(DummyView.full_height())
        .layer(entry_view)
        .with_name(Node::body().name());
    let padded_view = PaddedView::lrtb(0, 1, 0, 1, stack_view);

    let statusbar = render_statusbar(&mut siv);
    let layout = LinearLayout::vertical()
        .child(statusbar)
        .child(padded_view)
        .full_screen();

    siv.screen_mut().add_transparent_layer(layout);
    siv.run();
}
