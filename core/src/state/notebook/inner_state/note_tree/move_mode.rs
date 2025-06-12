use {
    super::NoteTreeState,
    crate::{
        Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
        db::CoreBackend,
        state::notebook::{InnerState, NotebookState, SelectedItem, directory, note},
        transition::{MoveModeTransition, NoteTreeTransition},
        types::{KeymapGroup, KeymapItem},
    },
};

pub async fn consume<B: CoreBackend + ?Sized>(
    db: &mut B,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;

    match event {
        Key(KeyEvent::J | KeyEvent::Down) => MoveModeTransition::SelectNext.into(),
        Key(KeyEvent::K | KeyEvent::Up) => MoveModeTransition::SelectPrev.into(),
        Key(KeyEvent::CapG) => MoveModeTransition::SelectLast.into(),
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
        _ => Err(Error::Todo(
            "Notebook::NoteTree::MoveMode::consume".to_owned(),
        )),
    }
}

impl From<MoveModeTransition> for Result<NotebookTransition> {
    fn from(transition: MoveModeTransition) -> Self {
        Ok(NotebookTransition::NoteTree(NoteTreeTransition::MoveMode(
            transition,
        )))
    }
}

pub fn keymap() -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            KeymapItem::new("j", "Select next"),
            KeymapItem::new("k", "Select previous"),
            KeymapItem::new("G", "Select last"),
            KeymapItem::new("Enter", "Move to selected directory"),
            KeymapItem::new("Esc", "Cancel"),
        ],
    )]
}
