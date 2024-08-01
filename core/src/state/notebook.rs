mod consume;
mod directory_item;

use {
    crate::{
        data::{Directory, Note},
        event::KeyEvent,
        state::GetInner,
        types::DirectoryId,
        Error, Event, Glues, NotebookTransition, Result,
    },
    consume::{directory, note, traverse},
};

pub use directory_item::{DirectoryItem, DirectoryItemChildren, TreeItem};

pub struct NotebookState {
    pub root: DirectoryItem,
    pub selected: SelectedItem,
    pub editing: Option<Note>,

    pub inner_state: InnerState,
}

pub enum SelectedItem {
    Note(Note),
    Directory(Directory),
    None,
}

pub enum InnerState {
    NoteSelected,
    NoteMoreActions,
    DirectorySelected,
    DirectoryMoreActions,
    EditingViewMode,
    EditingEditMode,
}
use InnerState::*;

impl NotebookState {
    pub async fn new(glues: &mut Glues) -> Result<Self> {
        let db = &mut glues.db;
        let root_directory = db.fetch_directory(glues.root_id.clone()).await?;
        let notes = db.fetch_notes(root_directory.id.clone()).await?;
        let directories = db
            .fetch_directories(root_directory.id.clone())
            .await?
            .into_iter()
            .map(|directory| DirectoryItem {
                directory,
                children: None,
            })
            .collect();

        let root = DirectoryItem {
            directory: root_directory,
            children: Some(DirectoryItemChildren { notes, directories }),
        };
        let selected = SelectedItem::Directory(root.directory.clone());

        Ok(Self {
            inner_state: DirectorySelected,
            root,
            selected,
            editing: None,
        })
    }

    pub fn check_opened(&self, directory_id: &DirectoryId) -> bool {
        matches!(
            self.root.find(directory_id),
            Some(&DirectoryItem {
                children: Some(_),
                ..
            })
        )
    }

    pub fn describe(&self) -> Result<String> {
        Ok(match &self.inner_state {
            NoteMoreActions => "Note actions dialog".to_owned(),
            DirectoryMoreActions => "Directory actions dialog".to_owned(),
            NoteSelected => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' selected")
            }
            DirectorySelected => {
                let name = &self.get_selected_directory()?.name;

                format!("Directory '{name}' selected")
            }
            EditingViewMode => "editing - view".to_owned(),
            EditingEditMode => "editing - edit".to_owned(),
        })
    }

    pub fn shortcuts(&self) -> Vec<&str> {
        match &self.inner_state {
            NoteSelected => {
                vec![
                    "[O] Open note",
                    "[H] Close parent directory",
                    "[J] Select next",
                    "[K] Select previous",
                    "[M] More actions",
                ]
            }
            DirectorySelected => {
                vec![
                    "[L] Toggle",
                    "[H] Close",
                    "[J] Select next",
                    "[K] Select previous",
                    "[M] More actions",
                ]
            }
            EditingViewMode => {
                vec!["[B] Browse note tree", "[E] Edit mode"]
            }
            EditingEditMode => {
                vec!["[Esc] View mode & Save note"]
            }
            _ => vec![],
        }
    }

    pub fn get_selected_note(&self) -> Result<&Note> {
        match &self.selected {
            SelectedItem::Note(ref note) => Ok(note),
            _ => Err(Error::Wip("selected note not found".to_owned())),
        }
    }

    pub fn get_selected_directory(&self) -> Result<&Directory> {
        match &self.selected {
            SelectedItem::Directory(ref directory) => Ok(directory),
            _ => Err(Error::Wip("selected directory not found".to_owned())),
        }
    }

    pub fn get_editing(&self) -> Result<&Note> {
        self.editing
            .as_ref()
            .ok_or_else(|| Error::Wip("editing is none".to_owned()))
    }
}

pub async fn consume(glues: &mut Glues, event: Event) -> Result<NotebookTransition> {
    let db = &mut glues.db;
    let state: &mut NotebookState = glues.state.get_inner_mut()?;

    match (event, &state.inner_state) {
        (Event::OpenDirectory(directory_id), DirectorySelected | NoteSelected) => {
            directory::open(db, state, directory_id).await
        }
        (Event::Key(KeyEvent::L) | Event::Key(KeyEvent::Right), DirectorySelected) => {
            let directory_id = &state.get_selected_directory()?.id;
            let directory_item = state.root.find(directory_id).ok_or(Error::Wip(
                "[Key::L] failed to find parent directory".to_owned(),
            ))?;

            if directory_item.children.is_none() {
                directory::open(db, state, directory_id.clone()).await
            } else {
                directory::close(state, directory_id.clone())
            }
        }
        (Event::CloseDirectory(directory_id), DirectorySelected | NoteSelected) => {
            directory::close(state, directory_id)
        }
        (Event::Key(KeyEvent::H) | Event::Key(KeyEvent::Left), DirectorySelected) => {
            let directory_id = state.get_selected_directory()?.id.clone();

            directory::close(state, directory_id)
        }
        (Event::Key(KeyEvent::H) | Event::Key(KeyEvent::Left), NoteSelected) => {
            let directory_id = &state.get_selected_note()?.directory_id;
            let directory_item = state.root.find(directory_id).ok_or(Error::Wip(
                "[Key::H] failed to find parent directory".to_owned(),
            ))?;
            let directory = directory_item.directory.clone();

            directory::close_by_note(state, directory)
        }
        (Event::Key(KeyEvent::J), DirectorySelected | NoteSelected) => traverse::select_next(state),
        (Event::Key(KeyEvent::K), DirectorySelected | NoteSelected) => traverse::select_prev(state),
        (Event::Key(KeyEvent::M), NoteSelected) => {
            let note = state.get_selected_note()?.clone();

            note::show_actions_dialog(state, note)
        }
        (Event::Key(KeyEvent::M), DirectorySelected) => {
            let directory = state.get_selected_directory()?.clone();

            directory::show_actions_dialog(state, directory)
        }
        (Event::CloseNoteActionsDialog, NoteMoreActions) => {
            let note = state.get_selected_note()?.clone();

            note::select(state, note)
        }
        (Event::CloseDirectoryActionsDialog, DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            directory::select(state, directory)
        }
        (Event::SelectNote(note), DirectorySelected | NoteSelected | EditingViewMode) => {
            note::select(state, note)
        }
        (Event::SelectDirectory(directory), DirectorySelected | NoteSelected | EditingViewMode) => {
            directory::select(state, directory)
        }
        (Event::RenameNote(new_name), NoteMoreActions) => {
            let note = state.get_selected_note()?.clone();

            note::rename(db, state, note, new_name).await
        }
        (Event::RemoveNote, NoteMoreActions) => {
            let note = state.get_selected_note()?.clone();

            note::remove(db, state, note).await
        }
        (Event::RenameDirectory(new_name), DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            directory::rename(db, state, directory, new_name).await
        }
        (Event::RemoveDirectory, DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            directory::remove(db, state, directory).await
        }
        (Event::AddNote(note_name), DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            note::add(db, state, directory, note_name).await
        }
        (Event::AddDirectory(directory_name), DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            directory::add(db, state, directory, directory_name).await
        }
        (Event::Key(KeyEvent::O) | Event::OpenNote, NoteSelected) => {
            let note = state.get_selected_note()?.clone();

            note::open(db, state, note).await
        }
        (Event::UpdateNoteContent(content), EditingViewMode) => {
            note::update_content(db, state, content).await
        }
        (Event::Key(KeyEvent::E) | Event::EditNote, EditingViewMode) => note::edit(state).await,
        (Event::Key(KeyEvent::B), EditingViewMode) => note::browse(state).await,
        (Event::Key(KeyEvent::Esc), EditingEditMode) => note::view(state).await,
        (Event::Cancel, NoteMoreActions) => {
            let note = state.get_selected_note()?.clone();

            note::select(state, note.clone())
        }
        (Event::Cancel, DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            directory::select(state, directory)
        }
        (event @ Event::Key(_), _) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
