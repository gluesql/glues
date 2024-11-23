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
            let directory_id = &state.get_selected_note()?.directory_id;
            let directory_item = state.root.find(directory_id).ok_or(Error::Wip(
                "[Key::H] failed to find parent directory".to_owned(),
            ))?;
            let directory = directory_item.directory.clone();

            directory::close(state, directory)
        }
        Key(KeyEvent::J | KeyEvent::Down) => traverse::select_next(state),
        Key(KeyEvent::K | KeyEvent::Up) => traverse::select_prev(state),
        Key(KeyEvent::M) => {
            let note = state.get_selected_note()?.clone();

            note::show_actions_dialog(state, note)
        }
        Notebook(SelectNote(note)) => note::select(state, note),
        Notebook(SelectDirectory(directory)) => directory::select(state, directory),
        Key(KeyEvent::L | KeyEvent::Enter) | Notebook(OpenNote) => {
            let note = state.get_selected_note()?.clone();

            note::open(db, state, note).await
        }
        Key(KeyEvent::Num(n)) => {
            state.inner_state = InnerState::NoteTreeNumber(n.into());

            Ok(NotebookTransition::None)
        }
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
