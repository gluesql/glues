use crate::{
    Error, Event, KeyEvent, NumKey, Result,
    state::notebook::{EditorState, InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition, VimKeymapKind},
    types::{KeymapGroup, KeymapItem},
};

pub fn consume(state: &mut NotebookState, n: usize, event: Event) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::Num(NumKey::Zero)) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            DeleteLineStart.into()
        }
        Key(KeyEvent::Num(n2)) => {
            state.inner_state = InnerState::Editor(EditorState::Normal(
                super::VimNormalState::Delete2(n, n2.into()),
            ));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::D) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            DeleteLines(n).into()
        }
        Key(KeyEvent::J | KeyEvent::Down) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            DeleteLines(n + 1).into()
        }
        Key(KeyEvent::K | KeyEvent::Up) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            DeleteLinesUp(n + 1).into()
        }
        Key(KeyEvent::B) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            DeleteWordBack(n).into()
        }
        Key(KeyEvent::E | KeyEvent::W) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));
            DeleteWordEnd(n).into()
        }
        Key(KeyEvent::H) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));
            DeleteCharsBack(n).into()
        }
        Key(KeyEvent::L) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));
            DeleteChars(n).into()
        }
        Key(KeyEvent::DollarSign) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            DeleteLineEnd(n).into()
        }
        Key(KeyEvent::I) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::DeleteInside(n)));

            DeleteInsideMode.into()
        }
        Key(KeyEvent::Esc) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            IdleMode.into()
        }
        Key(KeyEvent::CtrlH) => Ok(NotebookTransition::ShowVimKeymap(
            VimKeymapKind::NormalDelete,
        )),
        event @ Key(_) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            super::idle::consume(state, event)
        }
        _ => Err(Error::Todo(
            "Notebook::EditingNormalMode::Delete::consume".to_owned(),
        )),
    }
}

pub fn keymap(n: usize) -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            KeymapItem::new("i", "Enter delete inside mode"),
            KeymapItem::new("d", format!("Delete {n} lines")),
            KeymapItem::new("0", "Delete from start of line"),
            KeymapItem::new("b", "Delete previous word"),
            KeymapItem::new("e/w", "Delete to word end"),
            KeymapItem::new("h", "Delete previous character"),
            KeymapItem::new("l", "Delete next character"),
            KeymapItem::new("$", "Delete to line end"),
            KeymapItem::new("1-9", "Append steps"),
            KeymapItem::new("Ctrl+h", "Show Vim keymap"),
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
