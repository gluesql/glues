use crate::{
    Error, Event, KeyEvent, Result,
    state::notebook::{EditorState, InnerState, NotebookState},
    transition::{NormalModeTransition, NotebookTransition},
    types::{KeymapGroup, KeymapItem},
};

pub fn consume(state: &mut NotebookState, event: Event) -> Result<NotebookTransition> {
    use Event::*;
    use NormalModeTransition::*;

    match event {
        Key(KeyEvent::L) => {
            let i = state.tab_index.ok_or(Error::InvalidState(
                "[ToggleTabClose::L] tab index must exist".to_owned(),
            ))? + 1;

            state.tabs.splice(i.., []);
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            CloseRightTabs(i).into()
        }
        Key(KeyEvent::H) => {
            let i = state.tab_index.ok_or(Error::InvalidState(
                "[ToggleTabClose::H] tab index must exist".to_owned(),
            ))?;

            state.tab_index = Some(0);
            state.tabs.splice(..i, []);
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            CloseLeftTabs(i).into()
        }
        event @ Key(_) => {
            state.inner_state =
                InnerState::Editor(EditorState::Normal(super::VimNormalState::Idle));

            super::idle::consume(state, event)
        }
        _ => Err(Error::Todo(
            "Notebook::EditingNormalMode::ToggleTabClose::consume".to_owned(),
        )),
    }
}

pub fn keymap() -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            KeymapItem::new("h", "Close left tabs"),
            KeymapItem::new("l", "Close right tabs"),
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
