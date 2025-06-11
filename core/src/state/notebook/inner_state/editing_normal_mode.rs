use {
    super::VimVisualState,
    crate::{
        Error, Event, KeyEvent, NotebookEvent, NumKey, Result,
        db::Db,
        state::notebook::{InnerState, NoteTreeState, NotebookState, directory, note, tabs},
        transition::{
            NormalModeTransition, NotebookTransition, VimKeymapKind, VisualModeTransition,
        },
    },
};

#[derive(Clone, Copy)]
pub enum VimNormalState {
    Idle,
    Toggle,
    ToggleTabClose,
    Numbering(usize),
    Gateway,
    Yank(usize),
    Yank2(usize, usize),
    Delete(usize),
    Delete2(usize, usize),
    DeleteInside(usize),
    Change(usize),
    Change2(usize, usize),
    ChangeInside(usize),
    Scroll,
}

pub async fn consume(
    db: &mut Db,
    state: &mut NotebookState,
    vim_state: VimNormalState,
    event: Event,
) -> Result<NotebookTransition> {
    match vim_state {
        VimNormalState::Idle => consume_idle(state, event).await,
        VimNormalState::Toggle => consume_toggle(db, state, event).await,
        VimNormalState::ToggleTabClose => consume_toggle_tab_close(state, event).await,
        VimNormalState::Numbering(n) => consume_numbering(state, n, event).await,
        VimNormalState::Gateway => consume_gateway(state, event).await,
        VimNormalState::Yank(n) => consume_yank(state, n, event).await,
        VimNormalState::Yank2(n1, n2) => consume_yank2(state, n1, n2, event).await,
        VimNormalState::Delete(n) => consume_delete(state, n, event).await,
        VimNormalState::Delete2(n1, n2) => consume_delete2(state, n1, n2, event).await,
        VimNormalState::DeleteInside(n) => consume_delete_inside(state, n, event).await,
        VimNormalState::Change(n) => consume_change(state, n, event).await,
        VimNormalState::Change2(n1, n2) => consume_change2(state, n1, n2, event).await,
        VimNormalState::ChangeInside(n) => consume_change_inside(state, n, event).await,
        VimNormalState::Scroll => consume_scroll(state, event).await,
    }
}

async fn consume_idle(state: &mut NotebookState, event: Event) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;
    use NotebookEvent as NE;

    match event {
        Notebook(NE::SelectNote(note)) => note::select(state, note),
        Notebook(NE::SelectDirectory(directory)) => directory::select(state, directory),
        Key(KeyEvent::Tab) => {
            state.inner_state = InnerState::NoteTree(NoteTreeState::NoteSelected);

            Ok(NotebookTransition::BrowseNoteTree)
        }
        Key(KeyEvent::T) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Toggle);

            ToggleMode.into()
        }
        Key(KeyEvent::Z) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Scroll);

            ScrollMode.into()
        }
        Key(KeyEvent::P) => Paste.into(),
        Key(KeyEvent::U) => Undo.into(),
        Key(KeyEvent::CtrlR) => Redo.into(),
        Key(KeyEvent::J | KeyEvent::Down) => MoveCursorDown(1).into(),
        Key(KeyEvent::K | KeyEvent::Up) => MoveCursorUp(1).into(),
        Key(KeyEvent::H | KeyEvent::Left) => MoveCursorBack(1).into(),
        Key(KeyEvent::L | KeyEvent::Right) => MoveCursorForward(1).into(),
        Key(KeyEvent::W) => MoveCursorWordForward(1).into(),
        Key(KeyEvent::E) => MoveCursorWordEnd(1).into(),
        Key(KeyEvent::B) => MoveCursorWordBack(1).into(),
        Key(KeyEvent::Num(NumKey::Zero)) => MoveCursorLineStart.into(),
        Key(KeyEvent::DollarSign) => MoveCursorLineEnd.into(),
        Key(KeyEvent::Tilde) => SwitchCase.into(),
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
        Key(KeyEvent::C) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Change(1));

            ChangeMode.into()
        }
        Key(KeyEvent::X) => DeleteChars(1).into(),
        Key(KeyEvent::S) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteChars(1).into()
        }
        Key(KeyEvent::CapS) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteLines(1).into()
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

async fn consume_toggle(
    db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::L | KeyEvent::Right) => tabs::select_next(db, state).await,
        Key(KeyEvent::H | KeyEvent::Left) => tabs::select_prev(db, state).await,
        Key(KeyEvent::CapH) => tabs::move_prev(state),
        Key(KeyEvent::CapL) => tabs::move_next(state),
        Key(KeyEvent::X) => tabs::close(db, state).await,
        Key(KeyEvent::CapX) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::ToggleTabClose);

            ToggleTabCloseMode.into()
        }
        Key(KeyEvent::N) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            ToggleLineNumbers.into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            ToggleBrowser.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_toggle_tab_close(
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::L) => {
            let i = state.tab_index.ok_or(Error::Wip(
                "[ToggleTabClose::L] tab index must exist".to_owned(),
            ))? + 1;

            state.tabs.splice(i.., []);
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            CloseRightTabs(i).into()
        }
        Key(KeyEvent::H) => {
            let i = state.tab_index.ok_or(Error::Wip(
                "[ToggleTabClose::H] tab index must exist".to_owned(),
            ))?;

            state.tab_index = Some(0);
            state.tabs.splice(..i, []);
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            CloseLeftTabs(i).into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_numbering(
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
        Key(KeyEvent::J | KeyEvent::Down) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            MoveCursorDown(n).into()
        }
        Key(KeyEvent::K | KeyEvent::Up) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            MoveCursorUp(n).into()
        }
        Key(KeyEvent::H | KeyEvent::Left) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            NormalModeTransition::MoveCursorBack(n).into()
        }
        Key(KeyEvent::L | KeyEvent::Right) => {
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

            DeleteChars(n).into()
        }
        Key(KeyEvent::CapS) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteLines(n).into()
        }
        Key(KeyEvent::Y) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Yank(n));

            YankMode.into()
        }
        Key(KeyEvent::D) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Delete(n));

            DeleteMode.into()
        }
        Key(KeyEvent::C) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Change(n));

            ChangeMode.into()
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

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_gateway(state: &mut NotebookState, event: Event) -> Result<NotebookTransition> {
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

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_yank(
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

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_yank2(
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

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_delete(
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(NumKey::Zero)) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteLineStart.into()
        }
        Key(KeyEvent::Num(n2)) => {
            state.inner_state =
                InnerState::EditingNormalMode(VimNormalState::Delete2(n, n2.into()));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::D) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteLines(n).into()
        }
        Key(KeyEvent::J | KeyEvent::Down) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteLines(n + 1).into()
        }
        Key(KeyEvent::K | KeyEvent::Up) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteLinesUp(n + 1).into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteWordBack(n).into()
        }
        Key(KeyEvent::E) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
            DeleteWordEnd(n).into()
        }
        Key(KeyEvent::H) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
            DeleteCharsBack(n).into()
        }
        Key(KeyEvent::L) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
            DeleteChars(n).into()
        }
        Key(KeyEvent::DollarSign) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteLineEnd(n).into()
        }
        Key(KeyEvent::I) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::DeleteInside(n));

            DeleteInsideMode.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            IdleMode.into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(
            VimKeymapKind::NormalDelete,
        )),
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_delete2(
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
        Key(KeyEvent::J | KeyEvent::Down) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteLines(n1 * n2 + 1).into()
        }
        Key(KeyEvent::K | KeyEvent::Up) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteLinesUp(n1 * n2 + 1).into()
        }
        Key(KeyEvent::I) => {
            state.inner_state =
                InnerState::EditingNormalMode(VimNormalState::DeleteInside(n1 * n2));

            DeleteInsideMode.into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
            DeleteWordBack(n1 * n2).into()
        }
        Key(KeyEvent::E) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
            DeleteWordEnd(n1 * n2).into()
        }
        Key(KeyEvent::H) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
            DeleteCharsBack(n1 * n2).into()
        }
        Key(KeyEvent::L) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);
            DeleteChars(n1 * n2).into()
        }
        Key(KeyEvent::DollarSign) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            DeleteLineEnd(n1 * n2).into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            IdleMode.into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(
            VimKeymapKind::NormalDelete2,
        )),
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_delete_inside(
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

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_change(
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(NumKey::Zero)) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteLineStart.into()
        }
        Key(KeyEvent::Num(n2)) => {
            state.inner_state =
                InnerState::EditingNormalMode(VimNormalState::Change2(n, n2.into()));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::I) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::ChangeInside(n));

            ChangeInsideMode.into()
        }
        Key(KeyEvent::C) => {
            state.inner_state = InnerState::EditingInsertMode;
            DeleteLinesAndInsert(n).into()
        }
        Key(KeyEvent::E | KeyEvent::W) => {
            state.inner_state = InnerState::EditingInsertMode;
            DeleteWordEnd(n).into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingInsertMode;
            DeleteWordBack(n).into()
        }
        Key(KeyEvent::DollarSign) => {
            state.inner_state = InnerState::EditingInsertMode;
            DeleteLineEnd(n).into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(
            VimKeymapKind::NormalChange,
        )),
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_change2(
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
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Change2(n1, n2));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::I) => {
            let n = n1.saturating_mul(n2);
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::ChangeInside(n));

            ChangeInsideMode.into()
        }
        Key(KeyEvent::C) => {
            let n = n1.saturating_mul(n2);
            state.inner_state = InnerState::EditingInsertMode;
            DeleteLinesAndInsert(n).into()
        }
        Key(KeyEvent::E | KeyEvent::W) => {
            let n = n1.saturating_mul(n2);
            state.inner_state = InnerState::EditingInsertMode;
            DeleteWordEnd(n).into()
        }
        Key(KeyEvent::B) => {
            let n = n1.saturating_mul(n2);
            state.inner_state = InnerState::EditingInsertMode;
            DeleteWordBack(n).into()
        }
        Key(KeyEvent::DollarSign) => {
            let n = n1.saturating_mul(n2);
            state.inner_state = InnerState::EditingInsertMode;
            DeleteLineEnd(n).into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(
            VimKeymapKind::NormalChange2,
        )),
        Key(KeyEvent::Esc) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_change_inside(
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::W) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteInsideWord(n).into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

async fn consume_scroll(state: &mut NotebookState, event: Event) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Z | KeyEvent::Dot) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            ScrollCenter.into()
        }
        Key(KeyEvent::T | KeyEvent::Enter) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            ScrollTop.into()
        }
        Key(KeyEvent::B | KeyEvent::Dash) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            ScrollBottom.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(VimNormalState::Idle);

            consume_idle(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

impl From<NormalModeTransition> for Result<NotebookTransition> {
    fn from(transition: NormalModeTransition) -> Self {
        Ok(NotebookTransition::EditingNormalMode(transition))
    }
}
