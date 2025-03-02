use {
    super::NoteTreeState,
    crate::{
        Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
        state::notebook::{InnerState, NotebookState, SelectedItem, directory, note},
        transition::NoteTreeTransition,
    },
};

pub async fn consume(
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NotebookEvent::*;

    let reset_state = |state: &mut NotebookState| {
        match state.selected {
            SelectedItem::Note { .. } => {
                state.inner_state = InnerState::NoteTree(NoteTreeState::NoteSelected);
            }
            SelectedItem::Directory { .. } => {
                state.inner_state = InnerState::NoteTree(NoteTreeState::DirectorySelected);
            }
            SelectedItem::None => {}
        };
    };

    match event {
        Notebook(SelectNote(note)) => note::select(state, note),
        Notebook(SelectDirectory(directory)) => directory::select(state, directory),
        Key(KeyEvent::Num(n2)) => {
            let step = n2 + n.saturating_mul(10);
            state.inner_state = InnerState::NoteTree(NoteTreeState::Numbering(step));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::Esc) => {
            reset_state(state);
            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::J | KeyEvent::Down) => {
            reset_state(state);
            Ok(NotebookTransition::NoteTree(
                NoteTreeTransition::SelectNext(n),
            ))
        }
        Key(KeyEvent::K | KeyEvent::Up) => {
            reset_state(state);
            Ok(NotebookTransition::NoteTree(
                NoteTreeTransition::SelectPrev(n),
            ))
        }
        Key(KeyEvent::CapG) => {
            reset_state(state);
            Ok(NotebookTransition::NoteTree(NoteTreeTransition::SelectLast))
        }
        Key(KeyEvent::AngleBracketOpen) => {
            reset_state(state);
            Ok(NotebookTransition::NoteTree(
                NoteTreeTransition::ShrinkWidth(n),
            ))
        }
        Key(KeyEvent::AngleBracketClose) => {
            reset_state(state);
            Ok(NotebookTransition::NoteTree(
                NoteTreeTransition::ExpandWidth(n),
            ))
        }
        event @ Key(_) => {
            reset_state(state);
            Ok(NotebookTransition::Inedible(event))
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
