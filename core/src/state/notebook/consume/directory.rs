use {
    super::breadcrumb,
    crate::{
        data::Directory,
        db::Db,
        state::notebook::{
            DirectoryItem, DirectoryItemChildren, InnerState, NotebookState, SelectedItem,
        },
        transition::MoveModeTransition,
        types::DirectoryId,
        Error, NotebookTransition, Result,
    },
    async_recursion::async_recursion,
};

pub async fn open(
    db: &mut Db,
    state: &mut NotebookState,
    directory_id: DirectoryId,
) -> Result<NotebookTransition> {
    let item = state
        .root
        .find_mut(&directory_id)
        .ok_or(Error::Wip(format!(
            "[directory::open] directory not found: {directory_id}"
        )))?;

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

#[async_recursion(?Send)]
pub async fn open_all(
    db: &mut Db,
    state: &mut NotebookState,
    directory_id: DirectoryId,
) -> Result<NotebookTransition> {
    if state.check_opened(&directory_id) {
        return Ok(NotebookTransition::None);
    }

    let directory = db.fetch_directory(directory_id).await?;

    if state.root.directory.id != directory.id {
        open_all(db, state, directory.parent_id).await?;
    }
    open(db, state, directory.id).await
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
    if state.root.directory.id == directory.id {
        return Ok(NotebookTransition::Alert(
            "Cannot rename the root directory".to_owned(),
        ));
    }

    db.rename_directory(directory.id.clone(), new_name.clone())
        .await?;

    directory.name = new_name;
    state.root.rename_directory(&directory).ok_or(Error::Wip(
        "[directory::rename] failed to find directory".to_owned(),
    ))?;
    state.inner_state = InnerState::DirectorySelected;

    breadcrumb::update_breadcrumbs(db, state).await?;

    Ok(NotebookTransition::RenameDirectory(directory))
}

pub async fn remove(
    db: &mut Db,
    state: &mut NotebookState,
    directory: Directory,
) -> Result<NotebookTransition> {
    if state.root.directory.id == directory.id {
        return Ok(NotebookTransition::Alert(
            "Cannot remove the root directory".to_owned(),
        ));
    }

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

pub async fn move_directory(
    db: &mut Db,
    state: &mut NotebookState,
    target_directory_id: DirectoryId,
) -> Result<NotebookTransition> {
    let directory = state.get_selected_directory()?.clone();
    if directory.id == target_directory_id {
        state.inner_state = InnerState::DirectorySelected;

        return Ok(NotebookTransition::MoveMode(MoveModeTransition::Cancel));
    }

    db.move_directory(directory.id.clone(), target_directory_id.clone())
        .await?;
    close(state, state.root.directory.clone())?;
    open_all(db, state, target_directory_id).await?;

    state.selected = SelectedItem::Directory(directory);
    state.inner_state = InnerState::DirectorySelected;

    breadcrumb::update_breadcrumbs(db, state).await?;

    Ok(NotebookTransition::MoveMode(MoveModeTransition::Commit))
}
