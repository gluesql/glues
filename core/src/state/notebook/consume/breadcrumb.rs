use {
    super::directory,
    crate::{Result, backend::CoreBackend, state::notebook::NotebookState},
};

pub(super) async fn update_breadcrumbs<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
) -> Result<()> {
    let mut tree_items = state.root.tree_items(0);

    for idx in 0..state.tabs.len() {
        let (note_id, note_name, directory_id) = {
            let tab = &state.tabs[idx];
            (
                tab.note.id.clone(),
                tab.note.name.clone(),
                tab.note.directory_id.clone(),
            )
        };

        let mut location = tree_items
            .iter()
            .enumerate()
            .find_map(|(i, item)| (item.id == &note_id).then_some((i, item.depth)));

        if location.is_none() {
            directory::open_all(db, state, directory_id.clone()).await?;
            tree_items = state.root.tree_items(0);
            location = tree_items
                .iter()
                .enumerate()
                .find_map(|(i, item)| (item.id == &note_id).then_some((i, item.depth)));
        }

        let mut breadcrumb = vec![note_name.clone()];

        if let Some((i, mut depth)) = location {
            tree_items[0..i].iter().rev().for_each(|item| {
                if item.depth < depth {
                    depth = item.depth;

                    breadcrumb.push(item.name.to_owned());
                }
            });
            breadcrumb.reverse();
        }

        state.tabs[idx].breadcrumb = breadcrumb;
    }

    Ok(())
}
