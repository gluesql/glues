use crate::{
    Error, Event, NotebookEvent, NotebookTransition, Result,
    db::Db,
    state::notebook::{NotebookState, directory, note},
};

pub async fn consume(
    db: &mut Db,
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
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
