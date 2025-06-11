use crate::{
    Event, NotebookTransition, Result, db::CoreBackend, state::notebook::NotebookState,
    types::KeymapGroup,
};

mod directory_more_actions;
mod directory_selected;
mod gateway;
mod move_mode;
mod note_more_actions;
mod note_selected;
mod numbering;

#[derive(Clone, Copy)]
pub enum NoteTreeState {
    NoteSelected,
    NoteMoreActions,
    DirectorySelected,
    DirectoryMoreActions,
    Numbering(usize),
    GatewayMode,
    MoveMode,
}

pub async fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    tree_state: NoteTreeState,
    event: Event,
) -> Result<NotebookTransition> {
    use NoteTreeState::*;

    match tree_state {
        NoteSelected => note_selected::consume(db, state, event).await,
        DirectorySelected => directory_selected::consume(db, state, event).await,
        NoteMoreActions => note_more_actions::consume(db, state, event).await,
        DirectoryMoreActions => directory_more_actions::consume(db, state, event).await,
        Numbering(n) => numbering::consume(state, n, event).await,
        GatewayMode => gateway::consume(state, event).await,
        MoveMode => move_mode::consume(db, state, event).await,
    }
}

pub fn keymap(state: &NotebookState, tree_state: NoteTreeState) -> Vec<KeymapGroup> {
    use NoteTreeState::*;

    match tree_state {
        NoteSelected => note_selected::keymap(state),
        DirectorySelected => directory_selected::keymap(state),
        NoteMoreActions => note_more_actions::keymap(),
        DirectoryMoreActions => directory_more_actions::keymap(),
        Numbering(n) => numbering::keymap(n),
        GatewayMode => gateway::keymap(),
        MoveMode => move_mode::keymap(),
    }
}
