pub mod item_list;
mod more_actions;

use {
    crate::{actions, traits::*, Node},
    cursive::{
        event::EventResult,
        view::{Nameable, Resizable},
        views::{Button, DummyView, FocusTracker, LinearLayout, TextView},
        Cursive, View, With,
    },
    glues_core::{data::Directory, state::note_tree::DirectoryItem, types::DirectoryId},
    item_list::render_item_list,
    more_actions::render_more_actions,
    std::rc::Rc,
};

pub fn render_directory(siv: &mut Cursive, item: DirectoryItem) -> impl View {
    let directory = item.directory.clone();
    let directory_node = Node::note_tree().directory(&directory.id);
    let opened = siv.glues().check_opened(&directory.id);

    let directory_id = directory.id.clone();
    let button = Button::new_raw(directory.name.clone(), on_item_click(directory_id))
        .with_name(directory_node.name_button().name());

    let caret = TextView::new(get_caret(opened))
        .with_name(Node::note_tree().directory(&directory.id).caret().name());
    let more_actions = Button::new_raw("", on_more_click(directory.clone()))
        .with_name(directory_node.more_button().name());
    let item = LinearLayout::horizontal()
        .child(caret)
        .child(button)
        .child(DummyView.fixed_width(2))
        .child(more_actions)
        .wrap_with(FocusTracker::new)
        .on_focus(on_item_focus(item))
        .on_focus_lost(on_item_focus_lost(directory.id.clone()));

    let mut container = LinearLayout::vertical().child(item);
    if opened {
        let layout = render_item_list(siv, directory.id.clone());

        container.add_child(layout);
    }

    container.with_name(directory_node.name())
}

fn on_item_click(directory_id: DirectoryId) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv| {
        if siv.glues().check_opened(&directory_id) {
            actions::close_directory(siv, &directory_id);
        } else {
            actions::open_directory(siv, &directory_id);
        }
    }
}

fn on_more_click(directory: Directory) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv| {
        let dialog = render_more_actions(directory.clone());

        siv.add_layer(dialog);
    }
}

fn get_caret(opened: bool) -> &'static str {
    if opened {
        "▾ "
    } else {
        "▸ "
    }
}

fn on_item_focus(item: DirectoryItem) -> impl for<'a> Fn(&'a mut LinearLayout) -> EventResult {
    let item = Rc::new(item);

    move |_| {
        let item = Rc::clone(&item);
        let id = item.directory.id.clone();

        EventResult::with_cb(move |siv| {
            let item = item.as_ref().clone();
            actions::select_directory(siv, item);

            Node::note_tree()
                .directory(&id)
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
                .directory(&id)
                .more_button()
                .find(siv)
                .set_label_raw("");
        })
    }
}
