use super::VimNormalState;
use crate::{
    Error, Event, KeyEvent, Result,
    state::notebook::{InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition, VimKeymapKind},
    types::{KeymapGroup, KeymapItem},
};

pub async fn consume(
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

            super::idle::consume(state, event).await
        }
        _ => Err(Error::Todo("Notebook::consume".to_owned())),
    }
}

pub fn keymap(n: usize) -> Vec<KeymapGroup> {
    let items = vec![
        KeymapItem::new("j", format!("Move cursor {n} steps down")),
        KeymapItem::new("k", format!("Move cursor {n} steps up")),
        KeymapItem::new("h", format!("Move cursor {n} steps left")),
        KeymapItem::new("l", format!("Move cursor {n} steps right")),
        KeymapItem::new("0-9", "Append steps"),
        KeymapItem::new("Ctrl+h", "Show Vim keymap"),
        KeymapItem::new("Esc", "Cancel"),
    ];

    vec![KeymapGroup::new("General", items)]
}
