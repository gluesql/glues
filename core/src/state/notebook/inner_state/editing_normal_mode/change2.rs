use crate::{
    Error, Event, KeyEvent, Result,
    state::notebook::{InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition, VimKeymapKind},
    types::{KeymapGroup, KeymapItem},
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
                InnerState::EditingNormalMode(super::VimNormalState::Change2(n1, n2));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::I) => {
            let n = n1.saturating_mul(n2);
            state.inner_state =
                InnerState::EditingNormalMode(super::VimNormalState::ChangeInside(n));

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
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            IdleMode.into()
        }
        event @ Key(_) => {
            state.inner_state = InnerState::EditingNormalMode(super::VimNormalState::Idle);

            super::idle::consume(state, event).await
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

pub fn keymap(n1: usize, n2: usize) -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            if n1 == 1 {
                KeymapItem::new("c", format!("Delete {n2} lines and enter insert mode"))
            } else {
                KeymapItem::new("c", format!("Delete {n1}*{n2} lines and enter insert mode"))
            },
            KeymapItem::new("i", "Enter change inside mode"),
            KeymapItem::new("0-9", "Append steps"),
            KeymapItem::new("Ctrl+h", "Show Vim keymap"),
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
