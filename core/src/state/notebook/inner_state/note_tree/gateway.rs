use crate::{
    Error, Event, KeyEvent, NotebookTransition, Result,
    state::notebook::{InnerState, NoteTreeState, NotebookState, SelectedItem},
    transition::NoteTreeTransition,
    types::{KeymapGroup, KeymapItem},
};

pub fn consume(state: &mut NotebookState, event: Event) -> Result<NotebookTransition> {
    use Event::*;

    match event {
        Key(KeyEvent::G) => {
            state.inner_state = leave_gateway_mode(&state.selected)?;

            Ok(NotebookTransition::NoteTree(
                NoteTreeTransition::SelectFirst,
            ))
        }
        Key(KeyEvent::Esc) => {
            state.inner_state = leave_gateway_mode(&state.selected)?;

            Ok(NotebookTransition::None)
        }
        event @ Key(_) => {
            state.inner_state = leave_gateway_mode(&state.selected)?;

            Ok(NotebookTransition::Inedible(event))
        }
        _ => Err(Error::Todo("NoteTree::GatewayMode::consume".to_owned())),
    }
}

fn leave_gateway_mode(selected: &SelectedItem) -> Result<InnerState> {
    match selected {
        SelectedItem::Directory(_) => Ok(InnerState::NoteTree(NoteTreeState::DirectorySelected)),
        SelectedItem::Note(_) => Ok(InnerState::NoteTree(NoteTreeState::NoteSelected)),
        SelectedItem::None => Err(Error::Todo("cannot leave gateway mode".to_owned())),
    }
}

pub fn keymap() -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            KeymapItem::new("g", "Select first"),
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
