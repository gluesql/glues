use crate::{
    Error, Event, KeyEvent, Result,
    backend::CoreBackend,
    state::notebook::{InnerState, NotebookState, VimNormalState},
    transition::{NotebookTransition, VisualModeTransition},
    types::{KeymapGroup, KeymapItem},
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
        _ => Err(Error::Todo(
            "Notebook::EditingVisualMode::Gateway::consume".to_owned(),
        )),
    }
}

pub fn keymap() -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            KeymapItem::new("g", "Move cursor to top"),
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
