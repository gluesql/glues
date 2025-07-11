use crate::{
    Error, Event, KeyEvent, NumKey, Result,
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
        Key(KeyEvent::Num(NumKey::Zero)) => {
            state.inner_state = InnerState::EditingInsertMode;

            DeleteLineStart.into()
        }
        Key(KeyEvent::Num(n2)) => {
            state.inner_state =
                InnerState::EditingNormalMode(super::VimNormalState::Change2(n, n2.into()));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::I) => {
            state.inner_state =
                InnerState::EditingNormalMode(super::VimNormalState::ChangeInside(n));

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
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            super::idle::consume(state, event).await
        }
        _ => Err(Error::Todo(
            "Notebook::EditingNormalMode::Change::consume".to_owned(),
        )),
    }
}

pub fn keymap(n: usize) -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            KeymapItem::new("i", "Enter change inside mode"),
            KeymapItem::new("c", format!("Delete {n} lines and enter insert mode")),
            KeymapItem::new("Ctrl+h", "Show Vim keymap"),
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
