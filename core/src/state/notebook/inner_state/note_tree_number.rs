use crate::{
    db::Db,
    state::notebook::{directory, note, InnerState, NotebookState, SelectedItem},
    Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
};

pub async fn consume(
    _db: &mut Db,
    state: &mut NotebookState,
    n: usize,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NotebookEvent::*;

    let reset_state = |state: &mut NotebookState| {
        match state.selected {
            SelectedItem::Note { .. } => {
                state.inner_state = InnerState::NoteSelected;
            }
            SelectedItem::Directory { .. } => {
                state.inner_state = InnerState::DirectorySelected;
            }
            SelectedItem::None => {}
        };
    };

    match event {
        Notebook(SelectNote(note)) => note::select(state, note),
        Notebook(SelectDirectory(directory)) => directory::select(state, directory),
        Key(KeyEvent::Num(n2)) => {
            let step = n2 + n.saturating_mul(10);
            state.inner_state = InnerState::NoteTreeNumber(step);

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::Esc) => {
            reset_state(state);
            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::J | KeyEvent::Down) => {
            reset_state(state);
            Ok(NotebookTransition::SelectNext(n))
        }
        Key(KeyEvent::K | KeyEvent::Up) => {
            reset_state(state);
            Ok(NotebookTransition::SelectPrev(n))
        }
        event @ Key(_) => {
            reset_state(state);
            Ok(NotebookTransition::Inedible(event))
        }
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
