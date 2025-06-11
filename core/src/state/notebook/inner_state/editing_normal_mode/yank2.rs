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
    n1: usize,
    n2: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(n)) => {
            let n2 = n + n2.saturating_mul(10);
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Yank2(n1, n2));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::Y) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            YankLines(n1 * n2).into()
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
