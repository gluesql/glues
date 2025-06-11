use {
    super::VimVisualState,
    crate::{
        Event, Result,
        db::CoreBackend,
        state::notebook::{InnerState, NotebookState},
        transition::{NormalModeTransition, NotebookTransition},
    },
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
