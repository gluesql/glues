use crate::{
    Error, Event, KeyEvent, Result,
    db::CoreBackend,
    state::notebook::{InnerState, NotebookState, VimNormalState},
    transition::{NotebookTransition, VisualModeTransition},
};

pub async fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use VisualModeTransition::*;

    match event {
        Key(KeyEvent::G) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Idle);

            MoveCursorTop.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Idle);

            Ok(NotebookTransition::None)
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            super::idle::consume(db, state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
