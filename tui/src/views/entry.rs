use {
    crate::traits::*,
    cursive::{
        view::View,
        views::{Button, DummyView, LinearLayout, PaddedView, TextView},
        Cursive,
    },
    glues_core::EntryEvent,
};

pub fn render_entry() -> impl View {
    let csv = |siv: &mut Cursive| {
        siv.prompt("Path?", |siv, path| {
            siv.dispatch(EntryEvent::OpenCsv(path.to_owned()));
        })
    };
    let json = |siv: &mut Cursive| {
        siv.prompt("Path?", |siv, path| {
            siv.dispatch(EntryEvent::OpenJson(path.to_owned()));
        })
    };
    let file = |siv: &mut Cursive| {
        siv.prompt("Path?", |siv, path| {
            siv.dispatch(EntryEvent::OpenFile(path.to_owned()));
        })
    };
    let git = |siv: &mut Cursive| {
        siv.prompt("Path of the existing git repository root", |siv, path| {
            let path = path.to_owned();

            siv.prompt("Remote? (default: origin)", move |siv, remote| {
                let path = path.clone();
                let remote = if remote.is_empty() { "origin" } else { remote }.to_owned();

                siv.prompt("Branch? (default: main)", move |siv, branch| {
                    let branch = if branch.is_empty() { "main" } else { branch }.to_owned();
                    let event = EntryEvent::OpenGit {
                        path: path.clone(),
                        remote: remote.clone(),
                        branch,
                    };

                    siv.dispatch(event);
                })
            });
        })
    };

    let layout = LinearLayout::vertical()
        .child(TextView::new("Open Notes"))
        .child(Button::new(" Instant ", |siv| {
            siv.dispatch(EntryEvent::OpenMemory);
        }))
        .child(Button::new(" CSV     ", csv))
        .child(Button::new(" JSON    ", json))
        .child(Button::new(" File    ", file))
        .child(Button::new(" Git     ", git))
        .child(DummyView)
        .child(DummyView)
        .child(Button::new(" Quit    ", |siv| siv.quit()));

    PaddedView::lrtb(10, 10, 3, 3, layout)
}
