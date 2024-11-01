use {
    super::VimVisualState,
    crate::{
        db::Db,
        state::notebook::{directory, note, InnerState, NotebookState},
        transition::{
            NormalModeTransition, NotebookTransition, VimKeymapKind, VisualModeTransition,
        },
        Error, Event, KeyEvent, NotebookEvent, NumKey, Result,
    },
};

#[derive(Clone, Copy)]
pub enum VimNormalState {
    Idle,
    Numbering(usize),
    Gateway,
    Yank(usize),
    Yank2(usize, usize),
    Delete(usize),
    Delete2(usize, usize),
    DeleteInside(usize),
}

pub async fn consume(
    db: &mut Db,
    state: &mut NotebookState,
    vim_state: VimNormalState,
    event: Event,
) -> Result<NotebookTransition> {
    match vim_state {
        VimNormalState::Idle => consume_idle(db, state, event).await,
        VimNormalState::Numbering(n) => consume_numbering(db, state, n, event).await,
        VimNormalState::Gateway => consume_gateway(db, state, event).await,
        VimNormalState::Yank(n) => consume_yank(db, state, n, event).await,
        VimNormalState::Yank2(n1, n2) => consume_yank2(db, state, n1, n2, event).await,
        VimNormalState::Delete(n) => consume_delete(db, state, n, event).await,
        VimNormalState::Delete2(n1, n2) => consume_delete2(db, state, n1, n2, event).await,
        VimNormalState::DeleteInside(n) => consume_delete_inside(db, state, n, event).await,
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
        Key(KeyEvent::P) => Paste.into(),
        Key(KeyEvent::U) => Undo.into(),
        Key(KeyEvent::CtrlR) => Redo.into(),
        Key(KeyEvent::J) => MoveCursorDown(1).into(),
        Key(KeyEvent::K) => MoveCursorUp(1).into(),
        Key(KeyEvent::H) => MoveCursorBack(1).into(),
        Key(KeyEvent::L) => MoveCursorForward(1).into(),
        Key(KeyEvent::W) => MoveCursorWordForward(1).into(),
        Key(KeyEvent::E) => MoveCursorWordEnd(1).into(),
        Key(KeyEvent::B) => MoveCursorWordBack(1).into(),
        Key(KeyEvent::Num(NumKey::Zero)) => MoveCursorLineStart.into(),
        Key(KeyEvent::DollarSign) => MoveCursorLineEnd.into(),
        Key(KeyEvent::Caret) => MoveCursorLineNonEmptyStart.into(),
        Key(KeyEvent::CapG) => MoveCursorBottom.into(),
        Key(KeyEvent::I) => {
            state.inner_state = InnerState::EditingInsertMode;

            InsertAtCursor.into()
        }
        Key(KeyEvent::V) => {
            state.inner_state = InnerState::EditingVisualMode(VimVisualState::Idle);

            Ok(NotebookTransition::EditingVisualMode(
                VisualModeTransition::IdleMode,
            ))
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
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Gateway);

            GatewayMode.into()
        }
        Key(KeyEvent::Y) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Yank(1));

            YankMode.into()
        }
        Key(KeyEvent::D) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Delete(1));

            DeleteMode.into()
        }
        Key(KeyEvent::X) => DeleteChars(1).into(),
        Key(KeyEvent::S) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteCharsAndInsertMode(1).into()
        }
        Key(KeyEvent::CapS) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteLineAndInsertMode(1).into()
        }
        Key(KeyEvent::Num(n)) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Numbering(n.into()));

            NumberingMode.into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(VimKeymapKind::NormalIdle)),
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
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Numbering(step));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::J) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            MoveCursorDown(n).into()
        }
        Key(KeyEvent::K) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            MoveCursorUp(n).into()
        }
        Key(KeyEvent::H) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            NormalModeTransition::MoveCursorBack(n).into()
        }
        Key(KeyEvent::L) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            MoveCursorForward(n).into()
        }
        Key(KeyEvent::W) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            MoveCursorWordForward(n).into()
        }
        Key(KeyEvent::E) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            MoveCursorWordEnd(n).into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            MoveCursorWordBack(n).into()
        }
        Key(KeyEvent::CapG) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            MoveCursorToLine(n).into()
        }
        Key(KeyEvent::X) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteChars(n).into()
        }
        Key(KeyEvent::S) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteCharsAndInsertMode(n).into()
        }
        Key(KeyEvent::CapS) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteLineAndInsertMode(n).into()
        }
        Key(KeyEvent::Y) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Yank(n));

            YankMode.into()
        }
        Key(KeyEvent::D) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Delete(n));

            DeleteMode.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            IdleMode.into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(
            VimKeymapKind::NormalNumbering,
        )),
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

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
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            NormalModeTransition::MoveCursorTop.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(db, state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_yank(
    db: &mut Db,
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(n2)) if !matches!(n2, NumKey::Zero) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Yank2(n, n2.into()));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::Y) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            YankLines(n).into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(db, state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_yank2(
    db: &mut Db,
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
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Yank2(n1, n2));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::Y) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            YankLines(n1 * n2).into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(db, state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_delete(
    db: &mut Db,
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(n2)) if !matches!(n2, NumKey::Zero) => {
            state.inner_state =
                InnerState::EditingNormalMode(VimNormalState::Delete2(n, n2.into()));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::D) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteLines(n).into()
        }
        Key(KeyEvent::I) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::DeleteInside(n));

            DeleteInsideMode.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(db, state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_delete2(
    db: &mut Db,
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
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Delete2(n1, n2));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::D) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteLines(n1 * n2).into()
        }
        Key(KeyEvent::I) => {
            state.inner_state =
                InnerState::EditingNormalMode(VimNormalState::DeleteInside(n1 * n2));

            DeleteInsideMode.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(db, state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_delete_inside(
    db: &mut Db,
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::W) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteInsideWord(n).into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

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
