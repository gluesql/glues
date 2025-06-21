use crate::{
    Event, Result,
    backend::CoreBackend,
    state::notebook::NotebookState,
    transition::{NormalModeTransition, NotebookTransition},
    types::KeymapGroup,
};

mod change;
mod change2;
mod change_inside;
mod delete;
mod delete2;
mod delete_inside;
mod gateway;
mod idle;
mod numbering;
mod scroll;
mod toggle;
mod toggle_tab_close;
mod yank;
mod yank2;

#[derive(Clone, Copy)]
pub enum VimNormalState {
    Idle,
    Toggle,
    ToggleTabClose,
    Numbering(usize),
    Gateway,
    Yank(usize),
    Yank2(usize, usize),
    Delete(usize),
    Delete2(usize, usize),
    DeleteInside(usize),
    Change(usize),
    Change2(usize, usize),
    ChangeInside(usize),
    Scroll,
}

pub async fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    vim_state: VimNormalState,
    event: Event,
) -> Result<NotebookTransition> {
    match vim_state {
        VimNormalState::Idle => idle::consume(state, event).await,
        VimNormalState::Toggle => toggle::consume(db, state, event).await,
        VimNormalState::ToggleTabClose => toggle_tab_close::consume(state, event).await,
        VimNormalState::Numbering(n) => numbering::consume(state, n, event).await,
        VimNormalState::Gateway => gateway::consume(state, event).await,
        VimNormalState::Yank(n) => yank::consume(state, n, event).await,
        VimNormalState::Yank2(n1, n2) => yank2::consume(state, n1, n2, event).await,
        VimNormalState::Delete(n) => delete::consume(state, n, event).await,
        VimNormalState::Delete2(n1, n2) => delete2::consume(state, n1, n2, event).await,
        VimNormalState::DeleteInside(n) => delete_inside::consume(state, n, event).await,
        VimNormalState::Change(n) => change::consume(state, n, event).await,
        VimNormalState::Change2(n1, n2) => change2::consume(state, n1, n2, event).await,
        VimNormalState::ChangeInside(n) => change_inside::consume(state, n, event).await,
        VimNormalState::Scroll => scroll::consume(state, event).await,
    }
}

impl From<NormalModeTransition> for Result<NotebookTransition> {
    fn from(transition: NormalModeTransition) -> Self {
        Ok(NotebookTransition::EditingNormalMode(transition))
    }
}

pub fn keymap(vim_state: VimNormalState) -> Vec<KeymapGroup> {
    match vim_state {
        VimNormalState::Idle => idle::keymap(),
        VimNormalState::Toggle => toggle::keymap(),
        VimNormalState::ToggleTabClose => toggle_tab_close::keymap(),
        VimNormalState::Numbering(n) => numbering::keymap(n),
        VimNormalState::Gateway => gateway::keymap(),
        VimNormalState::Yank(n) => yank::keymap(n),
        VimNormalState::Yank2(n1, n2) => yank2::keymap(n1, n2),
        VimNormalState::Delete(n) => delete::keymap(n),
        VimNormalState::Delete2(n1, n2) => delete2::keymap(n1, n2),
        VimNormalState::DeleteInside(n) => delete_inside::keymap(n),
        VimNormalState::Change(n) => change::keymap(n),
        VimNormalState::Change2(n1, n2) => change2::keymap(n1, n2),
        VimNormalState::ChangeInside(n) => change_inside::keymap(n),
        VimNormalState::Scroll => scroll::keymap(),
    }
}
