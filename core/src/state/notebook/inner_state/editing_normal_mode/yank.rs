use super::VimNormalState;
use {
    super::VimVisualState,
    crate::{
        Error, Event, KeyEvent, NotebookEvent, NumKey, Result,
        db::CoreBackend,
        state::notebook::{InnerState, NoteTreeState, NotebookState, directory, note, tabs},
        transition::{
            NormalModeTransition, NotebookTransition, VimKeymapKind, VisualModeTransition,
        },
    },
};

pub async fn consume(
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(n2)) if !matches!(n2, NumKey::Zero) => {
            state.inner_state =
                InnerState::EditingNormalMode(super::VimNormalState::Yank2(n, n2.into()));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::Y) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            YankLines(n).into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            super::idle::consume(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
