use crate::{
    Error, Event, KeyEvent, Result,
    state::notebook::{InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition},
    types::{KeymapGroup, KeymapItem},
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
        _ => Err(Error::Todo("Notebook::consume".to_owned())),
    }
}

pub fn keymap() -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            KeymapItem::new("z|.", "Scroll to center"),
            KeymapItem::new("t|Enter", "Scroll to top"),
            KeymapItem::new("b|-", "Scroll to bottom"),
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
