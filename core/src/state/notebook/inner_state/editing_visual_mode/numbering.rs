use crate::{
    Error, Event, KeyEvent, Result,
    backend::CoreBackend,
    state::notebook::{InnerState, NotebookState, VimNormalState},
    transition::{NormalModeTransition, NotebookTransition, VimKeymapKind, VisualModeTransition},
    types::{KeymapGroup, KeymapItem},
};

pub fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use VisualModeTransition::*;

    match event {
        Key(KeyEvent::Num(n2)) => {
            let step = n2 + n.saturating_mul(10);
            state.inner_state =
                InnerState::EditingVisualMode(super::VimVisualState::Numbering(step));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::J | KeyEvent::Down) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Idle);

            MoveCursorDown(n).into()
        }
        Key(KeyEvent::K | KeyEvent::Up) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Idle);

            MoveCursorUp(n).into()
        }
        Key(KeyEvent::H | KeyEvent::Left) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Idle);

            NormalModeTransition::MoveCursorBack(n).into()
        }
        Key(KeyEvent::L | KeyEvent::Right) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Idle);

            MoveCursorForward(n).into()
        }
        Key(KeyEvent::W) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Idle);

            MoveCursorWordForward(n).into()
        }
        Key(KeyEvent::E) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Idle);

            MoveCursorWordEnd(n).into()
        }
        Key(KeyEvent::B) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Idle);

            MoveCursorWordBack(n).into()
        }
        Key(KeyEvent::CapG) => {
            state.inner_state = InnerState::EditingVisualMode(super::VimVisualState::Idle);

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

            super::idle::consume(db, state, event)
        }
        _ => Err(Error::Todo(
            "Notebook::EditingVisualMode::Numbering::consume".to_owned(),
        )),
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
