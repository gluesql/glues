use {
    super::{BrowsingState, DirectoryItem, NoteTreeState, EditingMode, EditingState},
    crate::{
        data::{Directory, Note},
        db::Db,
        Error, Result, Transition,
    },
};

pub(super) fn show_actions_dialog(state: &mut NoteTreeState, note: Note) -> Result<Transition> {
    state.inner_state = BrowsingState::NoteMoreActions(note.clone()).into();

    Ok(Transition::ShowNoteActionsDialog(note))
}

pub(super) fn select(state: &mut NoteTreeState, note: Note) -> Result<Transition> {
    state.inner_state = BrowsingState::NoteSelected(note).into();

    Ok(Transition::None)
}

pub(super) async fn rename(
    db: &mut Db,
    state: &mut NoteTreeState,
    mut note: Note,
    new_name: String,
) -> Result<Transition> {
    db.rename_note(note.id.clone(), new_name.clone()).await?;

    note.name = new_name;
    state.inner_state = BrowsingState::NoteSelected(note.clone()).into();

    Ok(Transition::RenameNote(note))
}

pub(super) async fn remove(
    db: &mut Db,
    state: &mut NoteTreeState,
    note: Note,
) -> Result<Transition> {
    db.remove_note(note.id.clone()).await?;

    state.inner_state = BrowsingState::NoteSelected(note.clone()).into();

    Ok(Transition::RemoveNote(note))
}

pub(super) async fn add(
    db: &mut Db,
    state: &mut NoteTreeState,
    directory: Directory,
    note_name: String,
) -> Result<Transition> {
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

    state.inner_state = BrowsingState::NoteSelected(note.clone()).into();

    Ok(Transition::AddNote(note))
}

pub(super) async fn open(
    db: &mut Db,
    state: &mut NoteTreeState,
    browsing_state: BrowsingState,
    note: Note,
) -> Result<Transition> {
    let content = db.fetch_note_content(note.id.clone()).await?;

    state.inner_state = EditingState {
        mode: EditingMode::View,
        browsing_state,
        note: note.clone(),
        content: content.clone(),
    }.into();

    Ok(Transition::OpenNote { note, content })
}

pub(super) async fn edit(
    state: &mut NoteTreeState,
    mut editing_state: EditingState,
) -> Result<Transition> {
    editing_state.mode = EditingMode::Edit;

    state.inner_state = editing_state.into();

    Ok(Transition::EditMode)
}

pub(super) async fn view(
    state: &mut NoteTreeState,
    mut editing_state: EditingState,
) -> Result<Transition> {
    let note = editing_state.note.clone();
    editing_state.mode = EditingMode::View;

    state.inner_state = editing_state.into();

    Ok(Transition::ViewMode(note))
}
