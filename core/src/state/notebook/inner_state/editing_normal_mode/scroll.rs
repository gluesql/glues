use crate::{
    Error, Event, KeyEvent, Result,
    state::notebook::{InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition},
};

pub async fn consume(state: &mut NotebookState, event: Event) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Z | KeyEvent::Dot) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            ScrollCenter.into()
        }
        Key(KeyEvent::T | KeyEvent::Enter) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            ScrollTop.into()
        }
        Key(KeyEvent::B | KeyEvent::Dash) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            ScrollBottom.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            super::idle::consume(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
