use crate::{
    Error, Event, NotebookEvent, NotebookTransition, Result,
    backend::CoreBackend,
    state::notebook::{NotebookState, note},
    types::{KeymapGroup, KeymapItem},
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

            Ok(note::select(state, note))
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

            Ok(note::select(state, note.clone()))
        }
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Todo(
            "Notebook::NoteTree::NoteMoreActions::consume".to_owned(),
        )),
    }
}

pub fn keymap() -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            KeymapItem::new("j", "Select next"),
            KeymapItem::new("k", "Select Previous"),
            KeymapItem::new("Enter", "Run selected item"),
            KeymapItem::new("Esc", "Close"),
        ],
    )]
}
