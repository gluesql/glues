use {
    super::directory,
    crate::{db::Db, state::notebook::NotebookState, Error, Result},
};

pub(super) async fn update_breadcrumbs(db: &mut Db, state: &mut NotebookState) -> Result<()> {
    let directory_ids = state
        .tabs
        .iter()
        .map(|tab| tab.note.directory_id.clone())
        .collect::<Vec<_>>();

    for directory_id in directory_ids {
        directory::open_all(db, state, directory_id).await?;
    }

    let tree_items = state.root.tree_items(0);

    for tab in state.tabs.iter_mut() {
        let (i, mut depth) = tree_items
            .iter()
            .enumerate()
            .find_map(|(i, item)| (item.id == &tab.note.id).then_some((i, item.depth)))
            .ok_or(Error::Wip(format!(
                "[breadcrumb::update_breadcrumbs] note not found: {}",
                tab.note.name
            )))?;

        let mut breadcrumb = vec![tab.note.name.clone()];
        tree_items[0..i].iter().rev().for_each(|item| {
            if item.depth < depth {
                depth = item.depth;

                breadcrumb.push(item.name.to_owned());
            }
        });
        breadcrumb.reverse();
        tab.breadcrumb = breadcrumb;
    }

    Ok(())
}
