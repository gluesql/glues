use {
    super::render_note,
    crate::cursive_ext::CursiveExt,
    async_io::block_on,
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

    let caret_id = format!("caret/{0}", directory.id);
    let item = LinearLayout::horizontal()
        .child(TextView::new(get_caret(opened)).with_name(caret_id))
        .child(button);

    let mut container = LinearLayout::vertical().child(item);
    if opened {
        let layout = render_items(siv, directory.id.clone());

        container.add_child(layout);
    }

    container.with_name(format!("container/{0}", directory.id))
}

fn render_items(siv: &mut Cursive, directory_id: DirectoryId) -> impl View {
    let directories = block_on(siv.glues().fetch_directories(directory_id.clone()));
    let notes = block_on(siv.glues().fetch_notes(directory_id.clone()));
    let mut layout = LinearLayout::vertical();

    for child in directories {
        layout.add_child(render_directory(siv, child));
    }

    for child in notes {
        layout.add_child(render_note(child));
    }

    PaddedView::lrtb(1, 0, 0, 0, layout)
}

fn on_item_click(directory_id: DirectoryId) -> impl for<'a> Fn(&'a mut Cursive) {
    move |siv| {
        let opened = siv.glues().check_opened(&directory_id);

        {
            let container_id = format!("container/{0}", &directory_id);
            let mut container = siv.find::<LinearLayout>(&container_id);

            if opened {
                siv.glues().close_directory(&directory_id);

                container.remove_child(1);
            } else {
                siv.glues().open_directory(directory_id.clone());

                let items = render_items(siv, directory_id.clone());
                container.add_child(items);
            }
        }

        let caret_id = format!("caret/{0}", directory_id);
        siv.find::<TextView>(&caret_id)
            .set_content(get_caret(!opened));
    }
}

fn get_caret(opened: bool) -> &'static str {
    if opened {
        "▾ "
    } else {
        "▸ "
    }
}
