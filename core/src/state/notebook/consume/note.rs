use {
    super::{breadcrumb, directory},
    crate::{
        data::{Directory, Note},
        db::Db,
        state::notebook::{
            DirectoryItem, InnerState, NotebookState, SelectedItem, Tab, VimNormalState,
        },
        transition::MoveModeTransition,
        types::{DirectoryId, NoteId},
        Error, NotebookTransition, Result,
    },
};

pub fn show_actions_dialog(state: &mut NotebookState, note: Note) -> Result<NotebookTransition> {
    state.inner_state = InnerState::NoteMoreActions;

    Ok(NotebookTransition::ShowNoteActionsDialog(note))
}

pub fn select(state: &mut NotebookState, note: Note) -> Result<NotebookTransition> {
    state.selected = SelectedItem::Note(note);
    state.inner_state = InnerState::NoteSelected;

    Ok(NotebookTransition::None)
}

pub async fn rename(
    db: &mut Db,
    state: &mut NotebookState,
    mut note: Note,
    new_name: String,
) -> Result<NotebookTransition> {
    db.rename_note(note.id.clone(), new_name.clone()).await?;
    db.log(
        "note::rename".to_owned(),
        format!("  id: {}\nfrom: {}\n  to: {}", note.id, note.name, new_name),
    )
    .await?;

    note.name = new_name;
    state.root.rename_note(&note).ok_or(Error::Wip(
        "[note::rename] failed to find parent directory".to_owned(),
    ))?;

    for tab in state.tabs.iter_mut().filter(|tab| tab.note.id == note.id) {
        tab.note.name = note.name.clone();
    }

    state.selected = SelectedItem::Note(note.clone());
    state.inner_state = InnerState::NoteSelected;

    breadcrumb::update_breadcrumbs(db, state).await?;

    Ok(NotebookTransition::RenameNote(note))
}

pub async fn remove(
    db: &mut Db,
    state: &mut NotebookState,
    note: Note,
) -> Result<NotebookTransition> {
    db.remove_note(note.id.clone()).await?;

    let directory = state.root.remove_note(&note).ok_or(Error::Wip(
        "[note::remove] failed to find parent directory".to_owned(),
    ))?;

    state.selected = SelectedItem::Directory(directory.clone());
    state.inner_state = InnerState::DirectorySelected;

    Ok(NotebookTransition::RemoveNote {
        note,
        selected_directory: directory.clone(),
    })
}

pub async fn add(
    db: &mut Db,
    state: &mut NotebookState,
    directory: Directory,
    note_name: String,
) -> Result<NotebookTransition> {
    let note = db.add_note(directory.id.clone(), note_name).await?;

    let item = state
        .root
        .find_mut(&directory.id)
        .ok_or(Error::Wip("todo: failed to find".to_owned()))?;

    if let DirectoryItem {
        children: Some(ref mut children),
        ..
    } = item
    {
        let notes = db.fetch_notes(directory.id.clone()).await?;
        children.notes = notes;
    }

    state.selected = SelectedItem::Note(note.clone());
    state.inner_state = InnerState::NoteSelected;

    Ok(NotebookTransition::AddNote(note))
}

pub async fn open(
    db: &mut Db,
    state: &mut NotebookState,
    note: Note,
) -> Result<NotebookTransition> {
    let content = db.fetch_note_content(note.id.clone()).await?;

    let i = state.tabs.iter().enumerate().find_map(|(i, tab)| {
        if tab.note.id == note.id {
            Some(i)
        } else {
            None
        }
    });

    if let Some(i) = i {
        state.tab_index = Some(i);
    } else {
        let tab = Tab {
            note: note.clone(),
            breadcrumb: vec![],
        };
        state.tabs.push(tab.clone());
        state.tab_index = Some(state.tabs.len() - 1);
    };

    state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

    breadcrumb::update_breadcrumbs(db, state).await?;

    Ok(NotebookTransition::OpenNote { note, content })
}

pub async fn view(state: &mut NotebookState) -> Result<NotebookTransition> {
    let note = state.get_editing()?.clone();

    state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

    Ok(NotebookTransition::ViewMode(note))
}

pub async fn update_content(
    db: &mut Db,
    note_id: NoteId,
    content: String,
) -> Result<NotebookTransition> {
    let current = db.fetch_note_content(note_id.clone()).await?;
    let content = content.trim_end();
    if current.trim_end() != content {
        db.update_note_content(note_id.clone(), content.to_owned())
            .await?;
    }

    Ok(NotebookTransition::UpdateNoteContent(note_id))
}

pub async fn move_note(
    db: &mut Db,
    state: &mut NotebookState,
    directory_id: DirectoryId,
) -> Result<NotebookTransition> {
    let mut note = state.get_selected_note()?.clone();
    note.directory_id = directory_id.clone();

    state.tabs.iter_mut().for_each(|tab| {
        if tab.note.id == note.id {
            tab.note.directory_id = directory_id.clone();
        }
    });

    db.move_note(note.id.clone(), directory_id.clone()).await?;
    directory::close(state, state.root.directory.clone())?;
    directory::open_all(db, state, directory_id).await?;

    state.selected = SelectedItem::Note(note);
    state.inner_state = InnerState::NoteSelected;

    breadcrumb::update_breadcrumbs(db, state).await?;

    Ok(NotebookTransition::MoveMode(MoveModeTransition::Commit))
}
