use crate::{
    Error, Event, KeyEvent, Result,
    state::notebook::{EditorState, InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition},
    types::{KeymapGroup, KeymapItem},
};

pub fn consume(
    state: &mut NotebookState,
    n1: usize,
    n2: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(n)) => {
            let n2 = n + n2.saturating_mul(10);
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Yank2(n1, n2)));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::Y) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            YankLines(n1 * n2).into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            super::idle::consume(state, event)
        }
        _ => Err(Error::Todo(
            "Notebook::EditingNormalMode::Yank2::consume".to_owned(),
        )),
    }
}

pub fn keymap(n1: usize, n2: usize) -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            if n1 == 1 {
                KeymapItem::new("y", format!("Yank {n2} lines"))
            } else {
                KeymapItem::new("y", format!("Yank {n1}*{n2} lines"))
            },
            KeymapItem::new("0-9", "Append steps"),
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
