use {
    cursive::{
        view::{Nameable, Resizable},
        views::{Button, LinearLayout, PaddedView, TextView, DummyView, HideableView},
        View,
        With,
        Cursive,
    },
    glues_core::{Glues, data::{Note}},
    async_io::block_on,
};

pub async fn note(siv: &mut Cursive, depth: usize, data: Note) -> impl View {
    let Note { name, id, .. } = data;
    /*
    let button_name = format!("{name}/button");
    */
    let actions_name = format!("{name}/actions");
    
    let button = Button::new_raw(name.clone(), move |siv| {
        let mut glues: &mut Glues = siv.user_data().unwrap();
        
        let content = glues.fetch_note_content(id.clone());
        let content = block_on(content);

        // println!("hey {content}/{id}");
        
        siv.call_on_name("temp_text", move |view: &mut TextView| {
            view.set_content(content);
        });
    });
    
    let name2 = name.clone();
    let more_actions = Button::new("More", move |siv| {
        let content = format!("{name2} - more action clicked");

        siv.call_on_name("temp_text", move |view: &mut TextView| {
            view.set_content(content);
        });
    })
    .wrap_with(HideableView::new)
    .hidden()
    .with_name(actions_name);
    
    let name2 = name.clone();
    let layout = LinearLayout::horizontal()
        .child(TextView::new("◦ "))
        .child(button)
        .child(DummyView.fixed_width(2))
        .child(more_actions)
        .wrap_with(cursive::views::FocusTracker::new)
        .on_focus(move |_| {
            let actions_name = format!("{name2}/actions");
            
            cursive::event::EventResult::with_cb(move |s| {
                s.call_on_name(actions_name.as_str(), |v: &mut HideableView<Button>| {
                    v.unhide();
                });
            })
        })
        .on_focus_lost(move |_| {
            let actions_name = format!("{name}/actions");
            
            cursive::event::EventResult::with_cb(move |s| {
                s.call_on_name(actions_name.as_str(), |v: &mut HideableView<Button>| {
                    v.hide();
                });
            })
        });
 
    // ;

    PaddedView::lrtb(depth, 0, 0, 0, layout)
}
