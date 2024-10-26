mod directory_more_actions;
mod directory_selected;
mod editing_edit_mode;
mod editing_view_mode;
mod note_more_actions;
mod note_selected;
mod note_tree_number;

use crate::{db::Db, state::notebook::NotebookState, Event, NotebookTransition, Result};

#[derive(Clone)]
pub enum InnerState {
    NoteSelected,
    NoteMoreActions,
    DirectorySelected,
    DirectoryMoreActions,
    NoteTreeNumber(usize),
    EditingViewMode,
    EditingEditMode,
}

pub async fn consume(
    db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use InnerState::*;

    match &state.inner_state {
        NoteSelected => note_selected::consume(db, state, event).await,
        DirectorySelected => directory_selected::consume(db, state, event).await,
        NoteMoreActions => note_more_actions::consume(db, state, event).await,
        DirectoryMoreActions => directory_more_actions::consume(db, state, event).await,
        NoteTreeNumber(n) => note_tree_number::consume(db, state, *n, event).await,
        EditingViewMode => editing_view_mode::consume(db, state, event).await,
        EditingEditMode => editing_edit_mode::consume(db, state, event).await,
    }
}
