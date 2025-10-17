use super::VimNormalState;
use crate::{
    Error, Event, KeyEvent, Result,
    state::notebook::{EditorState, InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition},
    types::{KeymapGroup, KeymapItem},
};

pub fn consume(state: &mut NotebookState, event: Event) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::G) => {
            state.inner_state = InnerState::Editor(EditorState::Normal(VimNormalState::Idle));

            NormalModeTransition::MoveCursorTop.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::Editor(EditorState::Normal(VimNormalState::Idle));

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::Editor(EditorState::Normal(VimNormalState::Idle));

            super::idle::consume(state, event)
        }
        _ => Err(Error::Todo(
            "Notebook::EditingNormalMode::Gateway::consume".to_owned(),
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
