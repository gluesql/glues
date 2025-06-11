use crate::{
    Error, Event, NotebookEvent, NotebookTransition, Result,
    db::CoreBackend,
    state::notebook::{NotebookState, note},
};

pub async fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NotebookEvent::*;

    match event {
        Notebook(CloseNoteActionsDialog) => {
            let note = state.get_selected_note()?.clone();

            note::select(state, note)
        }
        Notebook(RenameNote(new_name)) => {
            let note = state.get_selected_note()?.clone();

            note::rename(db, state, note, new_name).await
        }
        Notebook(RemoveNote) => {
            let note = state.get_selected_note()?.clone();

            note::remove(db, state, note).await
        }
        Cancel => {
            let note = state.get_selected_note()?.clone();

            note::select(state, note.clone())
        }
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
