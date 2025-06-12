use crate::{
    Error, Event, NotebookEvent, NotebookTransition, Result,
    db::CoreBackend,
    state::notebook::{NotebookState, directory, note},
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
        Notebook(CloseDirectoryActionsDialog) => {
            let directory = state.get_selected_directory()?.clone();

            directory::select(state, directory)
        }
        Notebook(RenameDirectory(new_name)) => {
            let directory = state.get_selected_directory()?.clone();

            directory::rename(db, state, directory, new_name).await
        }
        Notebook(RemoveDirectory) => {
            let directory = state.get_selected_directory()?.clone();

            directory::remove(db, state, directory).await
        }
        Notebook(AddNote(note_name)) => {
            let directory = state.get_selected_directory()?.clone();

            note::add(db, state, directory, note_name).await
        }
        Notebook(AddDirectory(directory_name)) => {
            let directory = state.get_selected_directory()?.clone();

            directory::add(db, state, directory, directory_name).await
        }
        Cancel => {
            let directory = state.get_selected_directory()?.clone();

            directory::select(state, directory)
        }
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Todo(
            "Notebook::NoteTree::DirectoryMoreActions::consume".to_owned(),
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
