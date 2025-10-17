use crate::{
    Error, Event, KeyEvent, NumKey, Result,
    state::notebook::{EditorState, InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition},
    types::{KeymapGroup, KeymapItem},
};

pub fn consume(state: &mut NotebookState, n: usize, event: Event) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(n2)) if !matches!(n2, NumKey::Zero) => {
            state.inner_state = InnerState::Editor(EditorState::Normal(
                super::VimNormalState::Yank2(n, n2.into()),
            ));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::Y) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            YankLines(n).into()
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
            "Notebook::EditingNormalMode::Yank::consume".to_owned(),
        )),
    }
}

pub fn keymap(n: usize) -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            KeymapItem::new("y", format!("Yank {n} lines")),
            KeymapItem::new("1-9", "Append steps"),
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
