use crate::{
    Error, Event, KeyEvent, NumKey, Result,
    db::CoreBackend,
    state::notebook::{InnerState, NotebookState, VimNormalState},
    transition::{NormalModeTransition, NotebookTransition, VimKeymapKind, VisualModeTransition},
};

pub async fn consume<B: CoreBackend + ?Sized>(
    _db: &mut B,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use VisualModeTransition::*;

    match event {
        Key(KeyEvent::J | KeyEvent::Down) => MoveCursorDown(1).into(),
        Key(KeyEvent::K | KeyEvent::Up) => MoveCursorUp(1).into(),
        Key(KeyEvent::H | KeyEvent::Left) => MoveCursorBack(1).into(),
        Key(KeyEvent::L | KeyEvent::Right) => MoveCursorForward(1).into(),
        Key(KeyEvent::W) => MoveCursorWordForward(1).into(),
        Key(KeyEvent::E) => MoveCursorWordEnd(1).into(),
        Key(KeyEvent::B) => MoveCursorWordBack(1).into(),
        Key(KeyEvent::Num(NumKey::Zero)) => MoveCursorLineStart.into(),
        Key(KeyEvent::DollarSign) => MoveCursorLineEnd.into(),
        Key(KeyEvent::Caret) => MoveCursorLineNonEmptyStart.into(),
        Key(KeyEvent::CapG) => MoveCursorBottom.into(),
        Key(KeyEvent::Tilde) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            SwitchCase.into()
        }
        Key(KeyEvent::U) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            ToLowercase.into()
        }
        Key(KeyEvent::CapU) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            ToUppercase.into()
        }
        Key(KeyEvent::D | KeyEvent::X) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteSelection.into()
        }

        Key(KeyEvent::S | KeyEvent::CapS) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteSelectionAndInsertMode.into()
        }
        Key(KeyEvent::Y) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            YankSelection.into()
        }
        Key(KeyEvent::G) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Gateway);

            GatewayMode.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::IdleMode,
            ))
        }
        Key(KeyEvent::Num(n)) => {
            state.inner_state =
                InnerState::EditingVisualMode(super::VimVisualState::Numbering(n.into()));

            NumberingMode.into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(VimKeymapKind::VisualIdle)),
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
