pub mod more_actions;

use {
    crate::{traits::*, Node},
    cursive::{
        event::EventResult,
        view::Nameable,
        views::{Button, FocusTracker, LinearLayout, TextView},
        Cursive, View, With,
    },
    glues_core::{data::Note, Event},
    std::rc::Rc,
};

pub fn render_note(note: Note) -> impl View {
    let note_node = Node::note_tree().note(&note.id);
    let button = Button::new_raw(note.name.clone(), on_item_click(note.id.clone()))
        .with_name(note_node.name_button().name());

    LinearLayout::horizontal()
        .child(TextView::new("◦ "))
        .child(button)
        .wrap_with(FocusTracker::new)
        .on_focus(on_item_focus(note.clone()))
        .with_name(note_node.name())
}

fn on_item_click(id: String) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv| {
        let content = siv.glues().db.fetch_note_content(id.clone()).log_unwrap();

        let mut editor = siv.find::<TextView>("temp_text");
        editor.set_content(content);
    }
}

fn on_item_focus(note: Note) -> impl for<'a> Fn(&'a mut LinearLayout) -> EventResult {
    let note = Rc::new(note);

    move |_| {
        let note = Rc::clone(&note);

        EventResult::with_cb(move |siv| {
            let note = note.as_ref().clone();

            siv.dispatch(Event::SelectNote(note));
        })
    }
}
