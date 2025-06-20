use {
    crate::{
        Error, NotebookTransition, Result,
        backend::CoreBackend,
        state::notebook::{
            InnerState, NoteTreeState, NotebookState, SelectedItem, VimNormalState, directory,
        },
        transition::NormalModeTransition,
    },
    std::cmp::min,
};

pub async fn select_prev<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
) -> Result<NotebookTransition> {
    state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

    let i = state
        .tab_index
        .ok_or(Error::InvalidState("opened note must exist".to_owned()))?;
    let i = if i == 0 { state.tabs.len() - 1 } else { i - 1 };
    state.tab_index = Some(i);

    let note = &state.tabs[i].note;
    state.selected = SelectedItem::Note(note.clone());

    let note_id = note.id.clone();
    let directory_id = note.directory_id.clone();

    directory::open_all(db, state, directory_id).await?;
    Ok(NotebookTransition::EditingNormalMode(
        NormalModeTransition::PrevTab(note_id),
    ))
}

pub async fn select_next<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
) -> Result<NotebookTransition> {
    state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

    let i = state
        .tab_index
        .ok_or(Error::InvalidState("opened note must exist".to_owned()))?;
    let i = if i + 1 >= state.tabs.len() { 0 } else { i + 1 };
    state.tab_index = Some(i);

    let note = &state.tabs[i].note;
    state.selected = SelectedItem::Note(note.clone());

    let note_id = note.id.clone();
    let directory_id = note.directory_id.clone();

    directory::open_all(db, state, directory_id).await?;
    Ok(NotebookTransition::EditingNormalMode(
        NormalModeTransition::NextTab(note_id),
    ))
}

pub fn move_prev(state: &mut NotebookState) -> Result<NotebookTransition> {
    let i = state
        .tab_index
        .ok_or(Error::InvalidState("opened note must exist".to_owned()))?;

    if i == 0 {
        return Ok(NotebookTransition::None);
    }

    let note = state.tabs.remove(i);
    state.tabs.insert(i - 1, note);
    state.tab_index = Some(i - 1);

    Ok(NotebookTransition::EditingNormalMode(
        NormalModeTransition::MoveTabPrev(i),
    ))
}

pub fn move_next(state: &mut NotebookState) -> Result<NotebookTransition> {
    let i = state
        .tab_index
        .ok_or(Error::InvalidState("opened note must exist".to_owned()))?;

    if i >= state.tabs.len() - 1 {
        return Ok(NotebookTransition::None);
    }

    let note = state.tabs.remove(i);
    state.tabs.insert(i + 1, note);
    state.tab_index = Some(i + 1);

    Ok(NotebookTransition::EditingNormalMode(
        NormalModeTransition::MoveTabNext(i),
    ))
}

pub async fn close<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
) -> Result<NotebookTransition> {
    state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
    let i = state
        .tab_index
        .ok_or(Error::InvalidState("opened note must exist".to_owned()))?;

    let note_id = state.tabs[i].note.id.clone();
    state.tabs.retain(|tab| tab.note.id != note_id);

    if state.tabs.is_empty() {
        state.tab_index = None;
        state.inner_state = InnerState::NoteTree(NoteTreeState::NoteSelected);

        return Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::CloseTab(note_id),
        ));
    }

    let i = min(i, state.tabs.len() - 1);
    state.tab_index = Some(i);

    let note = state.tabs[i].note.clone();
    state.selected = SelectedItem::Note(note.clone());

    directory::open_all(db, state, note.directory_id).await?;
    Ok(NotebookTransition::EditingNormalMode(
        NormalModeTransition::CloseTab(note_id),
    ))
}

pub async fn focus_editor<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
) -> Result<NotebookTransition> {
    let note = state.get_editing()?.clone();
    directory::open_all(db, state, note.directory_id.clone()).await?;

    state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
    state.selected = SelectedItem::Note(note);

    Ok(NotebookTransition::FocusEditor)
}
