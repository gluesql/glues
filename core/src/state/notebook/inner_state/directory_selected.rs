use crate::{
    db::Db,
    state::notebook::{directory, note, traverse, InnerState, NotebookState},
    Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
};

pub async fn consume(
    db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NotebookEvent::*;

    match event {
        Notebook(OpenDirectory(directory_id)) => directory::open(db, state, directory_id).await,
        Key(KeyEvent::L) | Key(KeyEvent::Right) => {
            let directory = state.get_selected_directory()?.clone();
            let directory_item = state.root.find(&directory.id).ok_or(Error::Wip(
                "[Key::L] failed to find the target directory".to_owned(),
            ))?;

            if directory_item.children.is_none() {
                directory::open(db, state, directory.id.clone()).await
            } else {
                directory::close(state, directory)
            }
        }
        Notebook(CloseDirectory(directory_id)) => {
            let directory = state
                .root
                .find(&directory_id)
                .ok_or(Error::Wip(
                    "[CloseDirectory] failed to find target directory".to_owned(),
                ))?
                .directory
                .clone();

            directory::close(state, directory)
        }
        Key(KeyEvent::H) | Key(KeyEvent::Left) => {
            let directory = state.get_selected_directory()?;
            if state.root.directory.id == directory.id {
                return Ok(NotebookTransition::None);
            }

            let parent_item = state.root.find(&directory.parent_id).ok_or(Error::Wip(
                "[Key::H] failed to find parent directory".to_owned(),
            ))?;
            let parent = parent_item.directory.clone();

            directory::close(state, parent)
        }
        Key(KeyEvent::J) => traverse::select_next(state),
        Key(KeyEvent::K) => traverse::select_prev(state),
        Key(KeyEvent::M) => {
            let directory = state.get_selected_directory()?.clone();

            directory::show_actions_dialog(state, directory)
        }
        Notebook(SelectNote(note)) => note::select(state, note),
        Notebook(SelectDirectory(directory)) => directory::select(state, directory),
        Key(KeyEvent::Num(n)) => {
            state.inner_state = InnerState::NoteTreeNumber(n.into());

            Ok(NotebookTransition::None)
        }
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
