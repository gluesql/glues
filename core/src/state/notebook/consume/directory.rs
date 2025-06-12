use {
    super::breadcrumb,
    crate::{
        Error, NotebookTransition, Result,
        data::Directory,
        db::CoreBackend,
        state::notebook::{
            DirectoryItem, DirectoryItemChildren, InnerState, NoteTreeState, NotebookState,
            SelectedItem,
        },
        transition::{MoveModeTransition, NoteTreeTransition},
        types::DirectoryId,
    },
};

pub async fn open<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    directory_id: DirectoryId,
) -> Result<NotebookTransition> {
    let item = state
        .root
        .find_mut(&directory_id)
        .ok_or(Error::NotFound(format!(
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

    Ok(NotebookTransition::NoteTree(
        NoteTreeTransition::OpenDirectory {
            id: directory_id,
            notes,
            directories,
        },
    ))
}

pub async fn open_all<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    directory_id: DirectoryId,
) -> Result<NotebookTransition> {
    if state.check_opened(&directory_id) {
        return Ok(NotebookTransition::None);
    }

    let mut path = Vec::new();
    let mut current_id = directory_id;
    loop {
        let directory = db.fetch_directory(current_id.clone()).await?;
        path.push(directory.id.clone());
        if state.root.directory.id == directory.id {
            break;
        }
        current_id = directory.parent_id;
    }

    path.reverse();

    let mut transition = NotebookTransition::None;
    for id in path {
        transition = open(db, state, id).await?;
    }

    Ok(transition)
}

pub fn close(state: &mut NotebookState, directory: Directory) -> Result<NotebookTransition> {
    state
        .root
        .find_mut(&directory.id)
        .ok_or(Error::NotFound(format!(
            "[directory::close] failed to find directory '{}'",
            directory.name
        )))?
        .children = None;

    let directory_id = directory.id.clone();

    state.selected = SelectedItem::Directory(directory);
    state.inner_state = InnerState::NoteTree(NoteTreeState::DirectorySelected);

    Ok(NotebookTransition::NoteTree(
        NoteTreeTransition::CloseDirectory(directory_id),
    ))
}

pub fn show_actions_dialog(
    state: &mut NotebookState,
    directory: Directory,
) -> Result<NotebookTransition> {
    state.selected = SelectedItem::Directory(directory.clone());
    state.inner_state = InnerState::NoteTree(NoteTreeState::DirectoryMoreActions);

    Ok(NotebookTransition::NoteTree(
        NoteTreeTransition::ShowDirectoryActionsDialog(directory),
    ))
}

pub fn select(state: &mut NotebookState, directory: Directory) -> Result<NotebookTransition> {
    state.selected = SelectedItem::Directory(directory);
    state.inner_state = InnerState::NoteTree(NoteTreeState::DirectorySelected);

    Ok(NotebookTransition::None)
}

pub async fn rename<B: CoreBackend + ?Sized>(
    db: &mut B,
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
    db.log(
        "directory::rename".to_owned(),
        format!(
            "  id: {}\nfrom: {}\n  to: {}",
            directory.id, directory.name, new_name
        ),
    )
    .await?;

    directory.name = new_name;
    state.root.rename_directory(&directory).ok_or(Error::NotFound(
        "[directory::rename] failed to find directory".to_owned(),
    ))?;
    state.inner_state = InnerState::NoteTree(NoteTreeState::DirectorySelected);

    breadcrumb::update_breadcrumbs(db, state).await?;

    Ok(NotebookTransition::NoteTree(
        NoteTreeTransition::RenameDirectory(directory),
    ))
}

pub async fn remove<B: CoreBackend + ?Sized>(
    db: &mut B,
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
        .ok_or(Error::NotFound(
            "[directory::remove] failed to find parent directory".to_owned(),
        ))?
        .clone();

    state.selected = SelectedItem::Directory(selected_directory.clone());
    state.inner_state = InnerState::NoteTree(NoteTreeState::DirectorySelected);

    Ok(NotebookTransition::NoteTree(
        NoteTreeTransition::RemoveDirectory {
            directory,
            selected_directory,
        },
    ))
}

pub async fn add<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    directory: Directory,
    directory_name: String,
) -> Result<NotebookTransition> {
    let parent_id = directory.id.clone();
    let directory = db.add_directory(parent_id.clone(), directory_name).await?;

    let item = state.root.find_mut(&parent_id).ok_or(Error::NotFound(format!(
        "[directory::add] parent directory not found: {}",
        parent_id
    )))?;

    if let DirectoryItem {
        children: Some(children),
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
    state.inner_state = InnerState::NoteTree(NoteTreeState::DirectorySelected);

    Ok(NotebookTransition::NoteTree(
        NoteTreeTransition::AddDirectory(directory),
    ))
}

pub async fn move_directory<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    target_directory_id: DirectoryId,
) -> Result<NotebookTransition> {
    let directory = state.get_selected_directory()?.clone();
    if directory.id == target_directory_id {
        state.inner_state = InnerState::NoteTree(NoteTreeState::DirectorySelected);

        return Ok(NotebookTransition::NoteTree(NoteTreeTransition::MoveMode(
            MoveModeTransition::Cancel,
        )));
    }

    db.move_directory(directory.id.clone(), target_directory_id.clone())
        .await?;
    close(state, state.root.directory.clone())?;
    open_all(db, state, target_directory_id).await?;

    state.selected = SelectedItem::Directory(directory);
    state.inner_state = InnerState::NoteTree(NoteTreeState::DirectorySelected);

    breadcrumb::update_breadcrumbs(db, state).await?;

    Ok(NotebookTransition::NoteTree(NoteTreeTransition::MoveMode(
        MoveModeTransition::Commit,
    )))
}
