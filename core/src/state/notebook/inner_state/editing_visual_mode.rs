use crate::{
    db::Db,
    state::notebook::{InnerState, NotebookState, VimNormalState},
    transition::{NormalModeTransition, NotebookTransition, VimKeymapKind, VisualModeTransition},
    Error, Event, KeyEvent, NumKey, Result,
};

#[derive(Clone, Copy)]
pub enum VimVisualState {
    Idle,
    Gateway,
    Numbering(usize),
}

pub async fn consume(
    db: &mut Db,
    state: &mut NotebookState,
    vim_state: VimVisualState,
    event: Event,
) -> Result<NotebookTransition> {
    match vim_state {
        VimVisualState::Idle => consume_idle(db, state, event).await,
        VimVisualState::Gateway => consume_gateway(db, state, event).await,
        VimVisualState::Numbering(n) => consume_numbering(db, state, n, event).await,
    }
}

async fn consume_idle(
    _db: &mut Db,
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
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Gateway);

            GatewayMode.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::IdleMode,
            ))
        }
        Key(KeyEvent::Num(n)) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Numbering(n.into()));

            NumberingMode.into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(VimKeymapKind::VisualIdle)),
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_gateway(
    db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use VisualModeTransition::*;

    match event {
        Key(KeyEvent::G) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            MoveCursorTop.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            Ok(NotebookTransition::None)
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(db, state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_numbering(
    db: &mut Db,
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use VisualModeTransition::*;

    match event {
        Key(KeyEvent::Num(n2)) => {
            let step = n2 + n.saturating_mul(10);
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Numbering(step));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::J | KeyEvent::Down) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            MoveCursorDown(n).into()
        }
        Key(KeyEvent::K | KeyEvent::Up) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            MoveCursorUp(n).into()
        }
        Key(KeyEvent::H | KeyEvent::Left) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            NormalModeTransition::MoveCursorBack(n).into()
        }
        Key(KeyEvent::L | KeyEvent::Right) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            MoveCursorForward(n).into()
        }
        Key(KeyEvent::W) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            MoveCursorWordForward(n).into()
        }
        Key(KeyEvent::E) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            MoveCursorWordEnd(n).into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            MoveCursorWordBack(n).into()
        }
        Key(KeyEvent::CapG) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            MoveCursorToLine(n).into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::IdleMode,
            ))
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(
            VimKeymapKind::VisualNumbering,
        )),
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(db, state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

impl From<VisualModeTransition> for Result<NotebookTransition> {
    fn from(transition: VisualModeTransition) -> Self {
        Ok(NotebookTransition::EditingVisualMode(transition))
    }
}
