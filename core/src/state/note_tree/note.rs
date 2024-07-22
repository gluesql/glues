use {
    super::{DirectoryItem, Editing, InnerState, NoteTreeState, SelectedItem},
    crate::{
        data::{Directory, Note},
        db::Db,
        Error, Result, Transition,
    },
};

pub(super) fn show_actions_dialog(state: &mut NoteTreeState, note: Note) -> Result<Transition> {
    state.inner_state = InnerState::NoteMoreActions;

    Ok(Transition::ShowNoteActionsDialog(note))
}

pub(super) fn select(state: &mut NoteTreeState, note: Note) -> Result<Transition> {
    state.selected = SelectedItem::Note(note);
    state.inner_state = InnerState::NoteSelected;

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
    state.selected = SelectedItem::Note(note.clone());
    state.inner_state = InnerState::NoteSelected;

    Ok(Transition::RenameNote(note))
}

pub(super) async fn remove(
    db: &mut Db,
    state: &mut NoteTreeState,
    note: Note,
) -> Result<Transition> {
    db.remove_note(note.id.clone()).await?;

    // TODO
    state.selected = SelectedItem::None;

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

    state.selected = SelectedItem::Note(note.clone());
    state.inner_state = InnerState::NoteSelected;

    Ok(Transition::AddNote(note))
}

pub(super) async fn open(db: &mut Db, state: &mut NoteTreeState, note: Note) -> Result<Transition> {
    let content = db.fetch_note_content(note.id.clone()).await?;

    state.editing = Some(Editing {
        note: note.clone(),
        content: content.clone(),
    });
    state.inner_state = InnerState::EditingViewMode;

    Ok(Transition::OpenNote { note, content })
}

pub(super) async fn edit(state: &mut NoteTreeState) -> Result<Transition> {
    state.inner_state = InnerState::EditingEditMode;

    Ok(Transition::EditMode)
}

pub(super) async fn view(state: &mut NoteTreeState) -> Result<Transition> {
    let note = state.get_editing()?.note.clone();

    state.inner_state = InnerState::EditingViewMode;

    Ok(Transition::ViewMode(note))
}
