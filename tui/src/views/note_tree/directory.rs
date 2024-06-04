use {
    super::render_note,
    crate::{traits::*, Node},
    cursive::{
        event::EventResult,
        view::Nameable,
        views::{Button, FocusTracker, LinearLayout, PaddedView, TextView},
        Cursive, View, With,
    },
    glues_core::{data::Directory, types::DirectoryId},
};

pub fn render_directory(siv: &mut Cursive, directory: Directory) -> impl View {
    let opened = siv.glues().check_opened(&directory.id);

    let directory_id = directory.id.clone();
    let button = Button::new_raw(directory.name.clone(), on_item_click(directory_id))
        .wrap_with(FocusTracker::new)
        .on_focus(|_| {
            EventResult::with_cb(|siv| {
                siv.find::<TextView>("temp_text").set_content("Focused");
            })
        });

    let caret = TextView::new(get_caret(opened))
        .with_name(Node::note_tree().directory(&directory.id).caret().name());
    let item = LinearLayout::horizontal().child(caret).child(button);

    let mut container = LinearLayout::vertical().child(item);
    if opened {
        let layout = render_items(siv, directory.id.clone());

        container.add_child(layout);
    }

    let nid = Node::note_tree().directory(&directory.id).name();
    container.with_name(nid)
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

fn get_caret(opened: bool) -> &'static str {
    if opened {
        "▾ "
    } else {
        "▸ "
    }
}
