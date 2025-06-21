use crate::{
    Error, Event, KeyEvent, Result,
    backend::CoreBackend,
    state::notebook::{InnerState, NotebookState, tabs},
    transition::{NormalModeTransition, NotebookTransition},
    types::{KeymapGroup, KeymapItem},
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
        _ => Err(Error::Todo(
            "Notebook::EditingNormalMode::Toggle::consume".to_owned(),
        )),
    }
}

pub fn keymap() -> Vec<KeymapGroup> {
    vec![
        KeymapGroup::new(
            "Tabs",
            vec![
                KeymapItem::new("h", "select left tab"),
                KeymapItem::new("l", "select right tab"),
                KeymapItem::new("H", "Move current tab to left"),
                KeymapItem::new("L", "Move current tab to right"),
                KeymapItem::new("x", "Close current tab"),
                KeymapItem::new("X", "Enter tab close mode"),
            ],
        ),
        KeymapGroup::new(
            "Options",
            vec![
                KeymapItem::new("b", "Toggle browser"),
                KeymapItem::new("n", "Toggle editor line number"),
                KeymapItem::new("Esc", "Cancel"),
            ],
        ),
    ]
}
