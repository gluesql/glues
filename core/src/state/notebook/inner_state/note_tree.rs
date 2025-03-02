use crate::{Event, NotebookTransition, Result, db::Db, state::notebook::NotebookState};

mod directory_more_actions;
mod directory_selected;
mod move_mode;
mod note_more_actions;
mod note_selected;
mod numbering;
mod gateway;

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

pub async fn consume(
    db: &mut Db,
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
