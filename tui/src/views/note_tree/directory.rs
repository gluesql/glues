mod more_actions;

use {
    super::render_note,
    crate::{traits::*, Node},
    cursive::{
        event::EventResult,
        view::{Nameable, Resizable},
        views::{Button, DummyView, FocusTracker, LinearLayout, PaddedView, TextView},
        Cursive, View, With,
    },
    glues_core::{data::Directory, types::DirectoryId},
    more_actions::render_more_actions,
};

pub fn render_directory(siv: &mut Cursive, directory: Directory) -> impl View {
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
        .on_focus(on_item_focus(directory.id.clone()))
        .on_focus_lost(on_item_focus_lost(directory.id.clone()));

    let mut container = LinearLayout::vertical().child(item);
    if opened {
        let layout = render_items(siv, directory.id.clone());

        container.add_child(layout);
    }

    container.with_name(directory_node.name())
}

fn render_items(siv: &mut Cursive, directory_id: DirectoryId) -> impl View {
    let directories = siv
        .glues()
        .fetch_directories(directory_id.clone())
        .log_unwrap();
    let notes = siv.glues().fetch_notes(directory_id.clone()).log_unwrap();
    let mut layout = LinearLayout::vertical();

    for child in directories {
        layout.add_child(render_directory(siv, child));
    }

    for child in notes {
        layout.add_child(render_note(child));
    }

    let nid = Node::note_tree()
        .directory(&directory_id)
        .note_list()
        .name();
    let layout = layout.with_name(nid);

    PaddedView::lrtb(1, 0, 0, 0, layout)
}

fn on_item_click(directory_id: DirectoryId) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv| {
        let opened = siv.glues().check_opened(&directory_id);
        let directory_node = Node::note_tree().directory(&directory_id);
        directory_node
            .caret()
            .find(siv)
            .set_content(get_caret(!opened));

        let mut container = directory_node.find(siv);

        if opened {
            siv.glues().close_directory(&directory_id);

            container.remove_child(1);
        } else {
            siv.glues().open_directory(directory_id.clone());

            let items = render_items(siv, directory_id.clone());
            container.add_child(items);
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

fn on_item_focus(id: String) -> impl for<'a> Fn(&'a mut LinearLayout) -> EventResult {
    move |_| {
        let id = id.clone();

        EventResult::with_cb(move |siv| {
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
