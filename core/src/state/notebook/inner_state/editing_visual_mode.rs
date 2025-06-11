use crate::{
    Event, Result,
    db::CoreBackend,
    state::notebook::NotebookState,
    transition::{NotebookTransition, VisualModeTransition},
    types::KeymapGroup,
};

mod gateway;
mod idle;
mod numbering;

#[derive(Clone, Copy)]
pub enum VimVisualState {
    Idle,
    Gateway,
    Numbering(usize),
}

pub async fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    vim_state: VimVisualState,
    event: Event,
) -> Result<NotebookTransition> {
    match vim_state {
        VimVisualState::Idle => idle::consume(db, state, event).await,
        VimVisualState::Gateway => gateway::consume(db, state, event).await,
        VimVisualState::Numbering(n) => numbering::consume(db, state, n, event).await,
    }
}

impl From<VisualModeTransition> for Result<NotebookTransition> {
    fn from(transition: VisualModeTransition) -> Self {
        Ok(NotebookTransition::EditingVisualMode(transition))
    }
}

pub fn keymap(vim_state: VimVisualState) -> Vec<KeymapGroup> {
    match vim_state {
        VimVisualState::Idle => idle::keymap(),
        VimVisualState::Gateway => gateway::keymap(),
        VimVisualState::Numbering(n) => numbering::keymap(n),
    }
}
