mod editing_insert_mode;
mod editing_normal_mode;
mod editing_visual_mode;
mod note_tree;

use crate::{
    Event, NotebookEvent, NotebookTransition, Result,
    db::Db,
    state::notebook::{NotebookState, note},
};
pub use editing_normal_mode::VimNormalState;
pub use editing_visual_mode::VimVisualState;
pub use note_tree::NoteTreeState;

#[derive(Clone, Copy)]
pub enum InnerState {
    NoteTree(NoteTreeState),

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
        NoteTree(tree_state) => note_tree::consume(db, state, *tree_state, event).await,
        EditingNormalMode(vim_state) => {
            editing_normal_mode::consume(db, state, *vim_state, event).await
        }
        EditingVisualMode(vim_state) => {
            editing_visual_mode::consume(db, state, *vim_state, event).await
        }
        EditingInsertMode => editing_insert_mode::consume(db, state, event).await,
    }
}
