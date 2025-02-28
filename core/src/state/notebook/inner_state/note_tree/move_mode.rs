use {
    super::NoteTreeState,
    crate::{
        db::Db,
        state::notebook::{directory, note, InnerState, NotebookState, SelectedItem},
        transition::{MoveModeTransition, NoteTreeTransition},
        Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
    },
};

pub async fn consume(
    db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;

    match event {
        Key(KeyEvent::J | KeyEvent::Down) => MoveModeTransition::SelectNext.into(),
        Key(KeyEvent::K | KeyEvent::Up) => MoveModeTransition::SelectPrev.into(),
        Key(KeyEvent::Esc) => {
            match state.selected {
                SelectedItem::Directory(_) => {
                    state.inner_state = InnerState::NoteTree(NoteTreeState::DirectorySelected);
                }
                SelectedItem::Note(_) => {
                    state.inner_state = InnerState::NoteTree(NoteTreeState::NoteSelected);
                }
                SelectedItem::None => {}
            };

            MoveModeTransition::Cancel.into()
        }
        Key(KeyEvent::Enter) => MoveModeTransition::RequestCommit.into(),
        Notebook(NotebookEvent::MoveNote(directory_id)) => {
            note::move_note(db, state, directory_id).await
        }
        Notebook(NotebookEvent::MoveDirectory(target_directory_id)) => {
            directory::move_directory(db, state, target_directory_id).await
        }
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

impl From<MoveModeTransition> for Result<NotebookTransition> {
    fn from(transition: MoveModeTransition) -> Self {
        Ok(NotebookTransition::NoteTree(NoteTreeTransition::MoveMode(
            transition,
        )))
    }
}
