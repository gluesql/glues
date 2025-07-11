use crate::{
    Error, Event, KeyEvent, Result,
    state::notebook::{InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition},
    types::{KeymapGroup, KeymapItem},
};

pub async fn consume(
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::W) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteInsideWord(n).into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            super::idle::consume(state, event).await
        }
        _ => Err(Error::Todo(
            "Notebook::EditingNormalMode::ChangeInside::consume".to_owned(),
        )),
    }
}

pub fn keymap(n: usize) -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            if n == 1 {
                KeymapItem::new("w", "Delete the current word and enter insert mode")
            } else {
                KeymapItem::new(
                    "w",
                    format!("Delete {n} words from cursor and enter insert mode"),
                )
            },
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
