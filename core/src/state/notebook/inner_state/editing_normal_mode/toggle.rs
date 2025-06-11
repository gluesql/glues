use crate::{
    Error, Event, KeyEvent, Result,
    db::CoreBackend,
    state::notebook::{InnerState, NotebookState, tabs},
    transition::{NormalModeTransition, NotebookTransition},
};

pub async fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::L | KeyEvent::Right) => tabs::select_next(db, state).await,
        Key(KeyEvent::H | KeyEvent::Left) => tabs::select_prev(db, state).await,
        Key(KeyEvent::CapH) => tabs::move_prev(state),
        Key(KeyEvent::CapL) => tabs::move_next(state),
        Key(KeyEvent::X) => tabs::close(db, state).await,
        Key(KeyEvent::CapX) => {
            state.inner_state =
                InnerState::EditingNormalMode(super::VimNormalState::ToggleTabClose);

            ToggleTabCloseMode.into()
        }
        Key(KeyEvent::N) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            ToggleLineNumbers.into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            ToggleBrowser.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            super::idle::consume(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
