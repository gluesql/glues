pub mod item_list;
pub mod more_actions;

use {
    crate::{traits::*, Node},
    cursive::{
        event::EventResult,
        view::{Nameable, Resizable},
        views::{Button, DummyView, FocusTracker, LinearLayout, TextView},
        Cursive, View, With,
    },
    glues_core::{
        data::Directory,
        state::note_tree::{DirectoryItem, NoteTreeState},
        types::DirectoryId,
        Event,
    },
    item_list::render_item_list,
};

pub fn render_directory(siv: &mut Cursive, item: DirectoryItem) -> impl View {
    let directory = item.directory.clone();
    let directory_node = Node::note_tree().directory(&directory.id);

    let directory_id = directory.id.clone();
    let button = Button::new_raw(directory.name.clone(), on_item_click(directory_id))
        .with_name(directory_node.name_button().name());

    let caret = TextView::new(get_caret(item.children.is_some()))
        .with_name(Node::note_tree().directory(&directory.id).caret().name());
    let content = LinearLayout::horizontal()
        .child(caret)
        .child(button)
        .child(DummyView.fixed_width(2))
        .wrap_with(FocusTracker::new)
        .on_focus(on_item_focus(directory.clone()));

    let mut container = LinearLayout::vertical().child(content);
    if let Some(children) = item.children {
        let layout = render_item_list(
            siv,
            directory.id.clone(),
            children.directories,
            children.notes,
        );

        container.add_child(layout);
    }

    container.with_name(directory_node.name())
}

fn on_item_click(directory_id: DirectoryId) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv| {
        if siv.state::<NoteTreeState>().check_opened(&directory_id) {
            siv.dispatch2(Event::CloseDirectory(directory_id.clone()))
        } else {
            siv.dispatch2(Event::OpenDirectory(directory_id.clone()))
        }
    }
}

fn get_caret(opened: bool) -> &'static str {
    if opened {
        "▾ "
    } else {
        "▸ "
    }
}

fn on_item_focus(directory: Directory) -> impl for<'a> Fn(&'a mut LinearLayout) -> EventResult {
    move |_| {
        let directory = directory.clone();

        EventResult::with_cb(move |siv| {
            siv.dispatch2(Event::SelectDirectory(directory.clone()));
        })
    }
}
