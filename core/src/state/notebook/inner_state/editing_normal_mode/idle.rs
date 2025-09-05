use super::VimNormalState;
use crate::state::notebook::inner_state::VimVisualState;
use crate::{
    Error, Event, KeyEvent, NotebookEvent, NumKey, Result,
    state::notebook::{InnerState, NoteTreeState, NotebookState, directory, note},
    transition::{NormalModeTransition, NotebookTransition, VimKeymapKind, VisualModeTransition},
    types::{KeymapGroup, KeymapItem},
};

pub fn consume(state: &mut NotebookState, event: Event) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;
    use NotebookEvent as NE;

    match event {
        Notebook(NE::SelectNote(note)) => Ok(note::select(state, note)),
        Notebook(NE::SelectDirectory(directory)) => Ok(directory::select(state, directory)),
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
        _ => Err(Error::Todo(
            "Notebook::EditingNormalMode::Idle::consume".to_owned(),
        )),
    }
}

pub fn keymap() -> Vec<KeymapGroup> {
    let items = vec![
        KeymapItem::new("Tab", "Browse notes"),
        KeymapItem::new("t", "Enter toggle-tabs mode"),
        KeymapItem::new("i", "Enter insert mode"),
        KeymapItem::new("v", "Enter visual mode"),
        KeymapItem::new("z", "Enter scroll mode"),
        KeymapItem::new("Ctrl+h", "Show Vim keymap"),
        KeymapItem::new("Esc", "Quit"),
    ];

    vec![KeymapGroup::new("General", items)]
}
