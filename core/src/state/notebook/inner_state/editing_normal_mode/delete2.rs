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
            state.inner_state =
                InnerState::EditingNormalMode(super::VimNormalState::Delete2(n1, n2));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::D) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            DeleteLines(n1 * n2).into()
        }
        Key(KeyEvent::J | KeyEvent::Down) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            DeleteLines(n1 * n2 + 1).into()
        }
        Key(KeyEvent::K | KeyEvent::Up) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            DeleteLinesUp(n1 * n2 + 1).into()
        }
        Key(KeyEvent::I) => {
            state.inner_state =
                InnerState::EditingNormalMode(super::VimNormalState::DeleteInside(n1 * n2));

            DeleteInsideMode.into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);
            DeleteWordBack(n1 * n2).into()
        }
        Key(KeyEvent::E) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);
            DeleteWordEnd(n1 * n2).into()
        }
        Key(KeyEvent::H) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);
            DeleteCharsBack(n1 * n2).into()
        }
        Key(KeyEvent::L) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);
            DeleteChars(n1 * n2).into()
        }
        Key(KeyEvent::DollarSign) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            DeleteLineEnd(n1 * n2).into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            IdleMode.into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(
            VimKeymapKind::NormalDelete2,
        )),
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            super::idle::consume(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
