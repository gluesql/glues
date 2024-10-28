use crate::{
    db::Db,
    state::notebook::{directory, note, InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition},
    Error, Event, KeyEvent, NotebookEvent, NumKey, Result,
};

#[derive(Clone, Copy)]
pub enum VimState {
    Idle,
    Numbering(usize),
    Gateway,
}

pub async fn consume(
    db: &mut Db,
    state: &mut NotebookState,
    vim_state: VimState,
    event: Event,
) -> Result<NotebookTransition> {
    match vim_state {
        VimState::Idle => consume_idle(db, state, event).await,
        VimState::Numbering(n) => consume_numbering(db, state, n, event).await,
        VimState::Gateway => consume_gateway(db, state, event).await,
    }
}

async fn consume_idle(
    db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;
    use NotebookEvent as NE;

    match event {
        Notebook(NE::SelectNote(note)) => note::select(state, note),
        Notebook(NE::SelectDirectory(directory)) => directory::select(state, directory),
        Notebook(NE::UpdateNoteContent(content)) => note::update_content(db, state, content).await,
        Key(KeyEvent::N) => {
            state.inner_state = InnerState::NoteSelected;

            Ok(NotebookTransition::BrowseNoteTree)
        }
        Key(KeyEvent::J) => MoveCursorDown(1).into(),
        Key(KeyEvent::K) => MoveCursorUp(1).into(),
        Key(KeyEvent::H) => MoveCursorBack(1).into(),
        Key(KeyEvent::L) => MoveCursorForward(1).into(),
        Key(KeyEvent::W) => MoveCursorWordForward(1).into(),
        Key(KeyEvent::E) => MoveCursorWordEnd(1).into(),
        Key(KeyEvent::B) => MoveCursorWordBack(1).into(),
        Key(KeyEvent::Num(NumKey::Zero)) => MoveCursorLineStart.into(),
        Key(KeyEvent::DollarSign) => MoveCursorLineEnd.into(),
        Key(KeyEvent::CapG) => MoveCursorBottom.into(),
        Key(KeyEvent::I) => {
            state.inner_state = InnerState::EditingInsertMode;

            InsertAtCursor.into()
        }
        Key(KeyEvent::CapI) => {
            state.inner_state = InnerState::EditingInsertMode;

            InsertAtLineStart.into()
        }
        Key(KeyEvent::A) => {
            state.inner_state = InnerState::EditingInsertMode;

            InsertAfterCursor.into()
        }
        Key(KeyEvent::CapA) => {
            state.inner_state = InnerState::EditingInsertMode;

            InsertAtLineEnd.into()
        }
        Key(KeyEvent::O) => {
            state.inner_state = InnerState::EditingInsertMode;

            InsertNewLineBelow.into()
        }
        Key(KeyEvent::CapO) => {
            state.inner_state = InnerState::EditingInsertMode;

            InsertNewLineAbove.into()
        }
        Key(KeyEvent::G) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Gateway);

            NumberingMode.into()
        }
        Key(KeyEvent::Num(n)) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Numbering(n.into()));

            NumberingMode.into()
        }
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
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
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(n2)) => {
            let step = n2 + n.saturating_mul(10);
            state.inner_state = InnerState::EditingNormalMode(VimState::Numbering(step));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::J) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            MoveCursorDown(n).into()
        }
        Key(KeyEvent::K) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            MoveCursorUp(n).into()
        }
        Key(KeyEvent::H) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            NormalModeTransition::MoveCursorBack(n).into()
        }
        Key(KeyEvent::L) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            MoveCursorForward(n).into()
        }
        Key(KeyEvent::W) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            MoveCursorWordForward(n).into()
        }
        Key(KeyEvent::E) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            MoveCursorWordEnd(n).into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            MoveCursorWordBack(n).into()
        }
        Key(KeyEvent::CapG) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            MoveCursorToLine(n).into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            consume_idle(db, state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_gateway(
    db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::G) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            NormalModeTransition::MoveCursorTop.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            consume_idle(db, state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

impl From<NormalModeTransition> for Result<NotebookTransition> {
    fn from(transition: NormalModeTransition) -> Self {
        Ok(NotebookTransition::EditingNormalMode(transition))
    }
}
