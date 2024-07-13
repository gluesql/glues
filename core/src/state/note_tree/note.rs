use {
    super::{BrowsingState, DirectoryItem, NoteTreeState},
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

pub(super) async fn edit(
    _db: &mut Db,
    _state: &mut NoteTreeState,
    _note: Note,
) -> Result<Transition> {
    panic!();
}
