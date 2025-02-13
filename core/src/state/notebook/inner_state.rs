mod directory_more_actions;
mod directory_selected;
mod editing_insert_mode;
mod editing_normal_mode;
mod editing_visual_mode;
mod move_mode;
mod note_more_actions;
mod note_selected;
mod note_tree_number;

use crate::{
    db::Db,
    state::notebook::{note, NotebookState},
    Event, NotebookEvent, NotebookTransition, Result,
};
pub use editing_normal_mode::VimNormalState;
pub use editing_visual_mode::VimVisualState;

#[derive(Clone, Copy)]
pub enum InnerState {
    NoteSelected,
    NoteMoreActions,
    DirectorySelected,
    DirectoryMoreActions,
    NoteTreeNumber(usize),
    MoveMode,
    EditingNormalMode(VimNormalState),
    EditingVisualMode(VimVisualState),
    EditingInsertMode,
}

pub async fn consume(
    db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use InnerState::*;

    if let Event::Notebook(NotebookEvent::UpdateNoteContent { note_id, content }) = event {
        return note::update_content(db, note_id, content).await;
    }

    match &state.inner_state {
        NoteSelected => note_selected::consume(db, state, event).await,
        DirectorySelected => directory_selected::consume(db, state, event).await,
        NoteMoreActions => note_more_actions::consume(db, state, event).await,
        DirectoryMoreActions => directory_more_actions::consume(db, state, event).await,
        NoteTreeNumber(n) => note_tree_number::consume(db, state, *n, event).await,
        MoveMode => move_mode::consume(db, state, event).await,
        EditingNormalMode(vim_state) => {
            editing_normal_mode::consume(db, state, *vim_state, event).await
        }
        EditingVisualMode(vim_state) => {
            editing_visual_mode::consume(db, state, *vim_state, event).await
        }
        EditingInsertMode => editing_insert_mode::consume(db, state, event).await,
    }
}
