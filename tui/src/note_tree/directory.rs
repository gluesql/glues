use {
    cursive::{
        view::Nameable,
        views::{Button, LinearLayout, PaddedView, TextView},
        View, With,
        Cursive,
    },
    glues_core::{Glues},
};

pub fn directory(_siv: &mut Cursive, depth: usize, name: String) -> impl View {
    let name2 = name.clone();

    let button = Button::new_raw(name.clone(), move |siv| {
        let name = &name2;

        siv.call_on_name("temp_text", |view: &mut TextView| {
            view.set_content("this should be");
        });
        
        let something = format!("something dynamic/{name}");
        
        siv.call_on_name(something.as_str(), |_view: &mut TextView| {
            
        });

        /*
        siv.call_on_name(name.as_str(), |view: &mut TextView| {
            let c = view.get_content();
            let c = c.deref().source();
            let c = if c == "▸ " {
                "▾ "
            } else {
                "▸ "
            };

            println!("ok = {c}");

            view.set_content("hello");
        });
        */
        siv.call_on_name(name.as_str(), |view: &mut TextView| {
            view.set_content("▸ ");
        });
    })
    .wrap_with(cursive::views::FocusTracker::new)
    .on_focus(|_| {
        cursive::event::EventResult::with_cb(|s| {
            s.call_on_name("temp_text", |v: &mut cursive::views::TextView| {
                v.set_content("Focused");
            });
        })
    })
    .on_focus_lost(|_| {
        cursive::event::EventResult::with_cb(|s| {
            s.call_on_name("temp_text", |v: &mut cursive::views::TextView| {
                // v.set_content("Focus lost");
            });
        })
    });

    /*
    , |siv: &mut Cursive| {
        siv.call_on_name("temp_text", |view: &mut TextView| {
            view.set_content("this should be");
        });

        /*
        siv.call_on_name(name, |view: &mut TextView| {
            let c = view.get_content();
            let c = c.deref().source();
            let c = if c == "▸ " {
                "▾ "
            } else {
                "▸ "
            };

            view.set_content(c);
        });
        */
    });
    */
    
    // ▸ ▾
    let item = LinearLayout::horizontal()
        .child(TextView::new("▾ ").with_name(name))
        .child(button)
        .with_name(format!("hey"));

    // .child(DummyView.full_width());
    let layout = LinearLayout::vertical()
        .child(item);
        /*
        .child(note(glues, 2, "great!"))
        .child(note(glues, 2, "seasons"));

    if depth < 2 {
        layout = layout.child(directory(glues, depth + 2, "sub directory".to_owned()));
    }
    */

    PaddedView::lrtb(depth, 0, 0, 0, layout)
}
