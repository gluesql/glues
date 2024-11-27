use {
    crate::{
        db::Db,
        state::notebook::{directory, InnerState, NotebookState, SelectedItem, VimNormalState},
        transition::NormalModeTransition,
        Error, NotebookTransition, Result,
    },
    std::cmp::min,
};

pub async fn select_prev(db: &mut Db, state: &mut NotebookState) -> Result<NotebookTransition> {
    state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

    let i = state
        .tab_index
        .ok_or(Error::Wip("opened note must exist".to_owned()))?;
    let i = if i + 1 >= state.tabs.len() { 0 } else { i + 1 };
    state.tab_index = Some(i);

    let note = &state.tabs[i];
    state.selected = SelectedItem::Note(note.clone());

    let note_id = note.id.clone();
    let directory_id = note.directory_id.clone();

    directory::open_all(db, state, directory_id).await?;
    Ok(NotebookTransition::EditingNormalMode(
        NormalModeTransition::NextTab(note_id),
    ))
}

pub async fn select_next(db: &mut Db, state: &mut NotebookState) -> Result<NotebookTransition> {
    state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

    let i = state
        .tab_index
        .ok_or(Error::Wip("opened note must exist".to_owned()))?;
    let i = if i == 0 { state.tabs.len() - 1 } else { i - 1 };
    state.tab_index = Some(i);

    let note = &state.tabs[i];
    state.selected = SelectedItem::Note(note.clone());

    let note_id = note.id.clone();
    let directory_id = note.directory_id.clone();

    directory::open_all(db, state, directory_id).await?;
    Ok(NotebookTransition::EditingNormalMode(
        NormalModeTransition::PrevTab(note_id),
    ))
}

pub async fn close(db: &mut Db, state: &mut NotebookState) -> Result<NotebookTransition> {
    state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
    let i = state
        .tab_index
        .ok_or(Error::Wip("opened note must exist".to_owned()))?;

    let note_id = state.tabs[i].id.clone();
    state.tabs.retain(|note| note.id != note_id);

    if state.tabs.is_empty() {
        state.tab_index = None;
        state.inner_state = InnerState::NoteSelected;

        return Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::CloseTab(note_id),
        ));
    }

    let i = min(i, state.tabs.len() - 1);
    state.tab_index = Some(i);

    let note = state.tabs[i].clone();
    state.selected = SelectedItem::Note(note.clone());

    directory::open_all(db, state, note.directory_id).await?;
    Ok(NotebookTransition::EditingNormalMode(
        NormalModeTransition::CloseTab(note_id),
    ))
}

pub async fn focus_editor(db: &mut Db, state: &mut NotebookState) -> Result<NotebookTransition> {
    let note = state.get_editing()?.clone();
    directory::open_all(db, state, note.directory_id.clone()).await?;

    state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
    state.selected = SelectedItem::Note(note);

    Ok(NotebookTransition::FocusEditor)
}
