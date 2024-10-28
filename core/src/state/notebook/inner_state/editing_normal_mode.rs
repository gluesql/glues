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
}

pub async fn consume(
    db: &mut Db,
    state: &mut NotebookState,
    vim_state: VimState,
    event: Event,
) -> Result<NotebookTransition> {
    match vim_state {
        VimState::Idle => consume_idle(db, state, event).await,
        VimState::Numbering(n) => consume_numbering(state, n, event).await,
    }
}

async fn consume_idle(
    db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NotebookEvent::*;

    match event {
        Notebook(SelectNote(note)) => note::select(state, note),
        Notebook(SelectDirectory(directory)) => directory::select(state, directory),
        Notebook(UpdateNoteContent(content)) => note::update_content(db, state, content).await,
        Notebook(BrowseNoteTree) => note::browse(state).await,
        Key(KeyEvent::J) => Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::MoveCursorDown(1),
        )),
        Key(KeyEvent::K) => Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::MoveCursorUp(1),
        )),
        Key(KeyEvent::H) => Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::MoveCursorBack(1),
        )),
        Key(KeyEvent::L) => Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::MoveCursorForward(1),
        )),
        Key(KeyEvent::W) => Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::MoveCursorWordForward(1),
        )),
        Key(KeyEvent::E) => Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::MoveCursorWordEnd(1),
        )),
        Key(KeyEvent::B) => Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::MoveCursorWordBack(1),
        )),
        Key(KeyEvent::Num(NumKey::Zero)) => Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::MoveCursorLineStart,
        )),
        Key(KeyEvent::DollarSign) => Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::MoveCursorLineEnd,
        )),
        Key(KeyEvent::CapG) => Ok(NotebookTransition::EditingNormalMode(
            NormalModeTransition::MoveCursorBottom,
        )),
        Key(KeyEvent::I) => {
            state.inner_state = InnerState::EditingInsertMode;

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::InsertAtCursor,
            ))
        }
        Key(KeyEvent::CapI) => {
            state.inner_state = InnerState::EditingInsertMode;

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::InsertAtLineStart,
            ))
        }
        Key(KeyEvent::A) => {
            state.inner_state = InnerState::EditingInsertMode;

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::InsertAfterCursor,
            ))
        }
        Key(KeyEvent::CapA) => {
            state.inner_state = InnerState::EditingInsertMode;

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::InsertAtLineEnd,
            ))
        }
        Key(KeyEvent::O) => {
            state.inner_state = InnerState::EditingInsertMode;

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::InsertNewLineBelow,
            ))
        }
        Key(KeyEvent::CapO) => {
            state.inner_state = InnerState::EditingInsertMode;

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::InsertNewLineAbove,
            ))
        }
        Key(KeyEvent::Num(n)) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Numbering(n.into()));

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::NumberingMode,
            ))
        }
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_numbering(
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;

    match event {
        Key(KeyEvent::Num(n2)) => {
            let step = n2 + n.saturating_mul(10);
            state.inner_state = InnerState::EditingNormalMode(VimState::Numbering(step));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::J) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::MoveCursorDown(n),
            ))
        }
        Key(KeyEvent::K) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::MoveCursorUp(n),
            ))
        }
        Key(KeyEvent::H) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::MoveCursorBack(n),
            ))
        }
        Key(KeyEvent::L) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::MoveCursorForward(n),
            ))
        }
        Key(KeyEvent::W) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::MoveCursorWordForward(n),
            ))
        }
        Key(KeyEvent::E) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::MoveCursorWordEnd(n),
            ))
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::MoveCursorWordBack(n),
            ))
        }
        Key(KeyEvent::CapG) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::MoveCursorToLine(n),
            ))
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimState::Idle);

            Ok(NotebookTransition::EditingNormalMode(
                NormalModeTransition::IdleMode,
            ))
        }
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
