use crate::{
    data::{Directory, Note},
    db::Db,
    state::notes::{DirectoryItem, InnerState, NotesState, SelectedItem},
    Error, Result, Transition,
};

pub fn show_actions_dialog(state: &mut NotesState, note: Note) -> Result<Transition> {
    state.inner_state = InnerState::NoteMoreActions;

    Ok(Transition::ShowNoteActionsDialog(note))
}

pub fn select(state: &mut NotesState, note: Note) -> Result<Transition> {
    state.selected = SelectedItem::Note(note);
    state.inner_state = InnerState::NoteSelected;

    Ok(Transition::None)
}

pub async fn rename(
    db: &mut Db,
    state: &mut NotesState,
    mut note: Note,
    new_name: String,
) -> Result<Transition> {
    db.rename_note(note.id.clone(), new_name.clone()).await?;

    note.name = new_name;
    state.selected = SelectedItem::Note(note.clone());
    state.inner_state = InnerState::NoteSelected;

    Ok(Transition::RenameNote(note))
}

pub async fn remove(db: &mut Db, state: &mut NotesState, note: Note) -> Result<Transition> {
    db.remove_note(note.id.clone()).await?;

    // TODO
    state.selected = SelectedItem::None;

    Ok(Transition::RemoveNote(note))
}

pub async fn add(
    db: &mut Db,
    state: &mut NotesState,
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

    state.selected = SelectedItem::Note(note.clone());
    state.inner_state = InnerState::NoteSelected;

    Ok(Transition::AddNote(note))
}

pub async fn open(db: &mut Db, state: &mut NotesState, note: Note) -> Result<Transition> {
    let content = db.fetch_note_content(note.id.clone()).await?;

    state.editing = Some(note.clone());
    state.inner_state = InnerState::EditingViewMode;

    Ok(Transition::OpenNote { note, content })
}

pub async fn edit(state: &mut NotesState) -> Result<Transition> {
    state.inner_state = InnerState::EditingEditMode;

    Ok(Transition::EditMode)
}

pub async fn view(state: &mut NotesState) -> Result<Transition> {
    let note = state.get_editing()?.clone();

    state.inner_state = InnerState::EditingViewMode;

    Ok(Transition::ViewMode(note))
}

pub async fn browse(state: &mut NotesState) -> Result<Transition> {
    let note = state.get_selected_note()?.clone();

    state.inner_state = InnerState::NoteSelected;

    Ok(Transition::SelectNote(note))
}

pub async fn update_content(
    db: &mut Db,
    state: &mut NotesState,
    content: String,
) -> Result<Transition> {
    let id = state.get_editing()?.id.clone();

    db.update_note_content(id, content).await?;

    Ok(Transition::UpdateNoteContent)
}
