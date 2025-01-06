use crate::{
    db::Db,
    state::notebook::{directory, InnerState, NotebookState, SelectedItem},
    transition::MoveModeTransition,
    Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
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
                    state.inner_state = InnerState::DirectorySelected;
                }
                SelectedItem::Note(_) => {
                    state.inner_state = InnerState::NoteSelected;
                }
                SelectedItem::None => {}
            };

            MoveModeTransition::Cancel.into()
        }
        Key(KeyEvent::Enter) => MoveModeTransition::RequestCommit.into(),
        Notebook(NotebookEvent::MoveNote(directory_id)) => {
            let note = state.get_selected_note()?.clone();

            db.move_note(note.id.clone(), directory_id.clone()).await?;
            directory::close(state, state.root.directory.clone())?;
            directory::open_all(db, state, directory_id).await?;

            state.selected = SelectedItem::Note(note);
            state.inner_state = InnerState::NoteSelected;
            MoveModeTransition::Commit.into()
        }
        Notebook(NotebookEvent::MoveDirectory(target_directory_id)) => {
            let directory = state.get_selected_directory()?.clone();
            if directory.id == target_directory_id {
                state.inner_state = InnerState::DirectorySelected;
                return MoveModeTransition::Cancel.into();
            }

            db.move_directory(directory.id.clone(), target_directory_id.clone())
                .await?;
            directory::close(state, state.root.directory.clone())?;
            directory::open_all(db, state, target_directory_id).await?;

            state.selected = SelectedItem::Directory(directory);
            state.inner_state = InnerState::DirectorySelected;
            MoveModeTransition::Commit.into()
        }
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}

impl From<MoveModeTransition> for Result<NotebookTransition> {
    fn from(transition: MoveModeTransition) -> Self {
        Ok(NotebookTransition::MoveMode(transition))
    }
}
