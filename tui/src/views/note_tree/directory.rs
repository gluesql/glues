pub mod item_list;
pub mod more_actions;

use {
    crate::{actions, traits::*, Node},
    cursive::{
        event::EventResult,
        view::{Nameable, Resizable},
        views::{Button, DummyView, FocusTracker, LinearLayout, TextView},
        Cursive, View, With,
    },
    glues_core::{
        state::note_tree::{DirectoryItem, NoteTreeState},
        types::DirectoryId,
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
        .on_focus(on_item_focus(directory.id.clone(), directory.name.clone()));

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
            actions::close_directory(siv, &directory_id);
        } else {
            actions::open_directory(siv, &directory_id);
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

fn on_item_focus(
    id: DirectoryId,
    name: String,
) -> impl for<'a> Fn(&'a mut LinearLayout) -> EventResult {
    move |_| {
        let id = id.clone();
        let name = name.clone();

        EventResult::with_cb(move |siv| {
            actions::select_directory(siv, id.clone(), name.clone());
        })
    }
}
