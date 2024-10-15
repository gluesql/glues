use crate::{
    data::Directory,
    db::Db,
    state::notebook::{
        DirectoryItem, DirectoryItemChildren, InnerState, NotebookState, SelectedItem,
    },
    types::DirectoryId,
    Error, NotebookTransition, Result,
};

pub async fn open(
    db: &mut Db,
    state: &mut NotebookState,
    directory_id: DirectoryId,
) -> Result<NotebookTransition> {
    let item = state.root.find_mut(&directory_id).ok_or(Error::Wip(
        "[state::notebook::open] directory not found".to_owned(),
    ))?;

    let notes = db.fetch_notes(directory_id.clone()).await?;
    let directories = db
        .fetch_directories(directory_id.clone())
        .await?
        .into_iter()
        .map(|directory| DirectoryItem {
            directory,
            children: None,
        })
        .collect::<Vec<_>>();

    item.children = Some(DirectoryItemChildren {
        notes: notes.clone(),
        directories: directories.clone(),
    });

    Ok(NotebookTransition::OpenDirectory {
        id: directory_id,
        notes,
        directories,
    })
}

pub fn close(state: &mut NotebookState, directory: Directory) -> Result<NotebookTransition> {
    state
        .root
        .find_mut(&directory.id)
        .ok_or(Error::Wip(format!(
            "[directory::close] failed to find directory '{}'",
            directory.name
        )))?
        .children = None;

    let directory_id = directory.id.clone();

    state.selected = SelectedItem::Directory(directory);
    state.inner_state = InnerState::DirectorySelected;

    Ok(NotebookTransition::CloseDirectory(directory_id))
}

pub fn show_actions_dialog(
    state: &mut NotebookState,
    directory: Directory,
) -> Result<NotebookTransition> {
    state.selected = SelectedItem::Directory(directory.clone());
    state.inner_state = InnerState::DirectoryMoreActions;

    Ok(NotebookTransition::ShowDirectoryActionsDialog(directory))
}

pub fn select(state: &mut NotebookState, directory: Directory) -> Result<NotebookTransition> {
    state.selected = SelectedItem::Directory(directory);
    state.inner_state = InnerState::DirectorySelected;

    Ok(NotebookTransition::None)
}

pub async fn rename(
    db: &mut Db,
    state: &mut NotebookState,
    mut directory: Directory,
    new_name: String,
) -> Result<NotebookTransition> {
    db.rename_directory(directory.id.clone(), new_name.clone())
        .await?;

    directory.name = new_name;
    state.root.rename_directory(&directory).ok_or(Error::Wip(
        "[directory::rename] failed to find directory".to_owned(),
    ))?;
    state.inner_state = InnerState::DirectorySelected;

    Ok(NotebookTransition::RenameDirectory(directory))
}

pub async fn remove(
    db: &mut Db,
    state: &mut NotebookState,
    directory: Directory,
) -> Result<NotebookTransition> {
    db.remove_directory(directory.id.clone()).await?;

    let selected_directory = state
        .root
        .remove_directory(&directory)
        .ok_or(Error::Wip(
            "[directory::remove] failed to find parent directory".to_owned(),
        ))?
        .clone();

    state.selected = SelectedItem::Directory(selected_directory.clone());
    state.inner_state = InnerState::DirectorySelected;

    Ok(NotebookTransition::RemoveDirectory {
        directory,
        selected_directory,
    })
}

pub async fn add(
    db: &mut Db,
    state: &mut NotebookState,
    directory: Directory,
    directory_name: String,
) -> Result<NotebookTransition> {
    let parent_id = directory.id.clone();
    let directory = db.add_directory(parent_id.clone(), directory_name).await?;

    let item = state
        .root
        .find_mut(&parent_id)
        .ok_or(Error::Wip("todo: failed to find {parent_id}".to_owned()))?;

    if let DirectoryItem {
        children: Some(ref mut children),
        ..
    } = item
    {
        let directories = db
            .fetch_directories(parent_id)
            .await?
            .into_iter()
            .map(|directory| DirectoryItem {
                directory,
                children: None,
            })
            .collect();

        children.directories = directories;
    }

    state.selected = SelectedItem::Directory(directory.clone());
    state.inner_state = InnerState::DirectorySelected;

    Ok(NotebookTransition::AddDirectory(directory))
}
