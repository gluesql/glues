use {
    super::NoteTreeState,
    crate::{
        Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
        db::Db,
        state::notebook::{InnerState, NotebookState, directory, note, tabs},
        transition::{MoveModeTransition, NoteTreeTransition},
    },
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
        Key(KeyEvent::J | KeyEvent::Down) => Ok(NotebookTransition::NoteTree(
            NoteTreeTransition::SelectNext(1),
        )),
        Key(KeyEvent::K | KeyEvent::Up) => Ok(NotebookTransition::NoteTree(
            NoteTreeTransition::SelectPrev(1),
        )),
        Key(KeyEvent::M) => {
            let note = state.get_selected_note()?.clone();

            note::show_actions_dialog(state, note)
        }
        Key(KeyEvent::Space) => {
            state.inner_state = InnerState::NoteTree(NoteTreeState::MoveMode);

            Ok(NotebookTransition::NoteTree(NoteTreeTransition::MoveMode(
                MoveModeTransition::Enter,
            )))
        }
        Notebook(SelectNote(note)) => note::select(state, note),
        Notebook(SelectDirectory(directory)) => directory::select(state, directory),
        Key(KeyEvent::L | KeyEvent::Enter) | Notebook(OpenNote) => {
            let note = state.get_selected_note()?.clone();

            note::open(db, state, note).await
        }
        Key(KeyEvent::Num(n)) => {
            state.inner_state = InnerState::NoteTree(NoteTreeState::Numbering(n.into()));

            Ok(NotebookTransition::None)
        }
        Key(KeyEvent::Tab) if !state.tabs.is_empty() => tabs::focus_editor(db, state).await,
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
