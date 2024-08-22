pub mod more_actions;

use {
    crate::{traits::*, Node},
    cursive::{
        event::EventResult,
        view::Nameable,
        views::{Button, FocusTracker, LinearLayout, NamedView, TextView},
        View, With,
    },
    glues_core::{data::Note, NotebookEvent},
    std::sync::Arc,
};

pub fn render_note(note: Note) -> impl View {
    let note_node = Node::notebook().note_tree().note(&note.id);
    let button = Button::new_raw(note.name.clone(), |siv| {
        siv.dispatch(NotebookEvent::OpenNote);
    })
    .with_name(note_node.name_button().name())
    .wrap_with(FocusTracker::new)
    .on_focus(on_item_focus(note.clone()));

    LinearLayout::horizontal()
        .child(TextView::new("◦ "))
        .child(button)
        .with_name(note_node.name())
}

fn on_item_focus(note: Note) -> impl for<'a> Fn(&'a mut NamedView<Button>) -> EventResult {
    let note = Arc::new(note);

    move |_| {
        let note = Arc::clone(&note);

        EventResult::with_cb(move |siv| {
            let note = note.as_ref().clone();

            siv.dispatch(NotebookEvent::SelectNote(note));
        })
    }
}
