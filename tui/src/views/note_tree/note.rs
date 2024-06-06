mod more_actions;

use {
    crate::{traits::*, Node},
    cursive::{
        event::EventResult,
        view::{Nameable, Resizable},
        views::{Button, DummyView, FocusTracker, LinearLayout, TextView},
        Cursive, View, With,
    },
    glues_core::data::Note,
    more_actions::render_more_actions,
};

pub fn render_note(note: Note) -> impl View {
    let note_node = Node::note_tree().note(&note.id);
    let button = Button::new_raw(note.name.clone(), on_item_click(note.id.clone()))
        .with_name(note_node.name_button().name());
    let more_actions =
        Button::new_raw("", on_more_click(note.clone())).with_name(note_node.more_button().name());

    LinearLayout::horizontal()
        .child(TextView::new("◦ "))
        .child(button)
        .child(DummyView.fixed_width(2))
        .child(more_actions)
        .wrap_with(FocusTracker::new)
        .on_focus(on_item_focus(note.id.clone()))
        .on_focus_lost(on_item_focus_lost(note.id.clone()))
        .with_name(Node::note_tree().note(&note.id).name())
}

fn on_item_click(id: String) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv| {
        let content = siv.glues().fetch_note_content(id.clone()).log_unwrap();

        let mut editor = siv.find::<TextView>("temp_text");
        editor.set_content(content);
    }
}

fn on_more_click(note: Note) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv| {
        let dialog = render_more_actions(note.clone());

        siv.add_layer(dialog);
    }
}

fn on_item_focus(id: String) -> impl for<'a> Fn(&'a mut LinearLayout) -> EventResult {
    move |_| {
        let id = id.clone();

        EventResult::with_cb(move |siv| {
            Node::note_tree()
                .note(&id)
                .more_button()
                .find(siv)
                .set_label("More");
        })
    }
}

fn on_item_focus_lost(id: String) -> impl for<'a> Fn(&'a mut LinearLayout) -> EventResult {
    move |_| {
        let id = id.clone();

        EventResult::with_cb(move |siv| {
            Node::note_tree()
                .note(&id)
                .more_button()
                .find(siv)
                .set_label_raw("");
        })
    }
}
