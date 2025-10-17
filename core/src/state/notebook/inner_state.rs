mod editor;
mod note_tree;

use crate::{
    Event, NotebookEvent, NotebookTransition, Result,
    backend::CoreBackend,
    state::notebook::{NotebookState, note},
    types::KeymapGroup,
};
pub use editor::{EditorState, VimNormalState, VimVisualState};
pub use note_tree::NoteTreeState;

#[derive(Clone, Copy)]
pub enum InnerState {
    NoteTree(NoteTreeState),
    Editor(EditorState),
}

pub async fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    if let Event::Notebook(NotebookEvent::UpdateNoteContent { note_id, content }) = event {
        return note::update_content(db, note_id, content).await;
    }

    match state.inner_state {
        InnerState::NoteTree(tree_state) => note_tree::consume(db, state, tree_state, event).await,
        InnerState::Editor(editor_state) => editor::consume(db, state, editor_state, event).await,
    }
}

pub fn keymap(state: &NotebookState) -> Vec<KeymapGroup> {
    match state.inner_state {
        InnerState::NoteTree(tree_state) => note_tree::keymap(state, tree_state),
        InnerState::Editor(editor_state) => editor::keymap(editor_state),
    }
}
