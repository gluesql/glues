use crate::{
    Error, Event, KeyEvent, NumKey, Result,
    state::notebook::{InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition, VimKeymapKind},
};

pub async fn consume(
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(NumKey::Zero)) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            DeleteLineStart.into()
        }
        Key(KeyEvent::Num(n2)) => {
            state.inner_state =
                InnerState::EditingNormalMode(super::VimNormalState::Delete2(n, n2.into()));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::D) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            DeleteLines(n).into()
        }
        Key(KeyEvent::J | KeyEvent::Down) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            DeleteLines(n + 1).into()
        }
        Key(KeyEvent::K | KeyEvent::Up) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            DeleteLinesUp(n + 1).into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            DeleteWordBack(n).into()
        }
        Key(KeyEvent::E) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);
            DeleteWordEnd(n).into()
        }
        Key(KeyEvent::H) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);
            DeleteCharsBack(n).into()
        }
        Key(KeyEvent::L) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);
            DeleteChars(n).into()
        }
        Key(KeyEvent::DollarSign) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            DeleteLineEnd(n).into()
        }
        Key(KeyEvent::I) => {
            state.inner_state =
                InnerState::EditingNormalMode(super::VimNormalState::DeleteInside(n));

            DeleteInsideMode.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            IdleMode.into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(
            VimKeymapKind::NormalDelete,
        )),
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            super::idle::consume(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
