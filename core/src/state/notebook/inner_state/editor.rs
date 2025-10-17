use crate::{
    Event, NotebookTransition, Result, backend::CoreBackend, state::notebook::NotebookState,
    types::KeymapGroup,
};

pub(super) mod insert_mode;
pub(super) mod normal_mode;
pub(super) mod visual_mode;

pub use normal_mode::VimNormalState;
pub use visual_mode::VimVisualState;

#[derive(Clone, Copy)]
pub enum EditorState {
    Normal(VimNormalState),
    Visual(VimVisualState),
    Insert,
}

pub async fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    editor_state: EditorState,
    event: Event,
) -> Result<NotebookTransition> {
    match editor_state {
        EditorState::Normal(vim_state) => normal_mode::consume(db, state, vim_state, event).await,
        EditorState::Visual(vim_state) => visual_mode::consume(db, state, vim_state, event),
        EditorState::Insert => insert_mode::consume(db, state, event),
    }
}

pub fn keymap(editor_state: EditorState) -> Vec<KeymapGroup> {
    match editor_state {
        EditorState::Normal(vim_state) => normal_mode::keymap(vim_state),
        EditorState::Visual(vim_state) => visual_mode::keymap(vim_state),
        EditorState::Insert => insert_mode::keymap(),
    }
}
