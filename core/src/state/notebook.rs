mod consume;
mod directory_item;

use {
    crate::{
        data::{Directory, Note},
        event::KeyEvent,
        state::{EntryState, GetInner},
        types::DirectoryId,
        Error, Event, Glues, NotebookEvent, NotebookTransition, Result,
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

#[derive(Clone)]
pub enum InnerState {
    NoteSelected,
    NoteMoreActions,
    DirectorySelected,
    DirectoryMoreActions,
    NoteTreeNumber(usize),
    EditingViewMode,
    EditingEditMode,
    EntryDialog(Box<InnerState>),
}
use InnerState::*;

impl NotebookState {
    pub async fn new(glues: &mut Glues) -> Result<Self> {
        let db = glues
            .db
            .as_mut()
            .ok_or(Error::Wip("[NotebookState::new] empty db".to_owned()))?;
        let root_id = db.root_id.clone();
        let root_directory = db.fetch_directory(root_id).await?;
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
            NoteTreeNumber(n) => {
                format!("Steps: '{n}' selected")
            }
            EditingViewMode => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' view mode")
            }
            EditingEditMode => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' edit mode")
            }
            EntryDialog(_) => "Global menu dialog".to_owned(),
        })
    }

    pub fn shortcuts(&self) -> Vec<String> {
        match &self.inner_state {
            NoteSelected => {
                vec![
                    "[o] Open note".to_owned(),
                    "[h] Close parent".to_owned(),
                    "[j] Down".to_owned(),
                    "[k] Up".to_owned(),
                    "[1-9] Set steps".to_owned(),
                    "[m] More actions".to_owned(),
                    "[Esc] Quit".to_owned(),
                ]
            }
            DirectorySelected => {
                vec![
                    "[l] Toggle".to_owned(),
                    "[h] Close parent".to_owned(),
                    "[j] Down".to_owned(),
                    "[k] Up".to_owned(),
                    "[1-9] Set steps".to_owned(),
                    "[m] More actions".to_owned(),
                    "[Esc] Quit".to_owned(),
                ]
            }
            NoteTreeNumber(n) => {
                vec![
                    format!("[j] Move {n} down"),
                    format!("[k] Move {n} up"),
                    "[0-9] Append steps".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingViewMode => {
                vec![
                    "[i] Edit mode".to_owned(),
                    "[b] Browse note tree".to_owned(),
                    "[n] Toggle line number".to_owned(),
                    "[h] Show editor keymap".to_owned(),
                    "[Esc] Quit".to_owned(),
                ]
            }
            EditingEditMode => {
                vec![
                    "[Esc] Save note & View mode".to_owned(),
                    "[Ctrl+h] Show editor keymap".to_owned(),
                ]
            }
            DirectoryMoreActions | NoteMoreActions | EntryDialog(_) => {
                vec![
                    "[j] Next".to_owned(),
                    "[k] Previous".to_owned(),
                    "[Enter] Select".to_owned(),
                    "[Esc] Close".to_owned(),
                ]
            }
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
    use Event::*;
    use NotebookEvent::*;

    let db = glues
        .db
        .as_mut()
        .ok_or(Error::Wip("[consume] empty db".to_owned()))?;
    let state: &mut NotebookState = glues.state.get_inner_mut()?;

    match (event, &state.inner_state) {
        (Notebook(CloseEntryDialog), EntryDialog(inner_state)) => {
            state.inner_state = *inner_state.clone();

            Ok(NotebookTransition::None)
        }
        (event, EntryDialog(_)) => EntryState::consume(glues, event)
            .await
            .map(NotebookTransition::Entry),
        (Key(KeyEvent::Esc), DirectorySelected | NoteSelected | EditingViewMode) => {
            state.inner_state = EntryDialog(Box::new(state.inner_state.clone()));

            Ok(NotebookTransition::ShowEntryDialog)
        }
        (Notebook(OpenDirectory(directory_id)), DirectorySelected | NoteSelected) => {
            directory::open(db, state, directory_id).await
        }
        (Key(KeyEvent::L) | Key(KeyEvent::Right), DirectorySelected) => {
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
        (Notebook(CloseDirectory(directory_id)), DirectorySelected | NoteSelected) => {
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
        (Key(KeyEvent::H) | Key(KeyEvent::Left), DirectorySelected) => {
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
        (Key(KeyEvent::H) | Key(KeyEvent::Left), NoteSelected) => {
            let directory_id = &state.get_selected_note()?.directory_id;
            let directory_item = state.root.find(directory_id).ok_or(Error::Wip(
                "[Key::H] failed to find parent directory".to_owned(),
            ))?;
            let directory = directory_item.directory.clone();

            directory::close(state, directory)
        }
        (Key(KeyEvent::J), DirectorySelected | NoteSelected) => traverse::select_next(state),
        (Key(KeyEvent::K), DirectorySelected | NoteSelected) => traverse::select_prev(state),
        (Key(KeyEvent::M), NoteSelected) => {
            let note = state.get_selected_note()?.clone();

            note::show_actions_dialog(state, note)
        }
        (Key(KeyEvent::M), DirectorySelected) => {
            let directory = state.get_selected_directory()?.clone();

            directory::show_actions_dialog(state, directory)
        }
        (Notebook(CloseNoteActionsDialog), NoteMoreActions) => {
            let note = state.get_selected_note()?.clone();

            note::select(state, note)
        }
        (Notebook(CloseDirectoryActionsDialog), DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            directory::select(state, directory)
        }
        (
            Notebook(SelectNote(note)),
            DirectorySelected | NoteSelected | NoteTreeNumber(_) | EditingViewMode,
        ) => note::select(state, note),
        (
            Notebook(SelectDirectory(directory)),
            DirectorySelected | NoteSelected | NoteTreeNumber(_) | EditingViewMode,
        ) => directory::select(state, directory),
        (Notebook(RenameNote(new_name)), NoteMoreActions) => {
            let note = state.get_selected_note()?.clone();

            note::rename(db, state, note, new_name).await
        }
        (Notebook(RemoveNote), NoteMoreActions) => {
            let note = state.get_selected_note()?.clone();

            note::remove(db, state, note).await
        }
        (Notebook(RenameDirectory(new_name)), DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            directory::rename(db, state, directory, new_name).await
        }
        (Notebook(RemoveDirectory), DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            directory::remove(db, state, directory).await
        }
        (Notebook(AddNote(note_name)), DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            note::add(db, state, directory, note_name).await
        }
        (Notebook(AddDirectory(directory_name)), DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            directory::add(db, state, directory, directory_name).await
        }
        (Key(KeyEvent::O) | Notebook(OpenNote), NoteSelected) => {
            let note = state.get_selected_note()?.clone();

            note::open(db, state, note).await
        }
        (Notebook(UpdateNoteContent(content)), EditingViewMode) => {
            note::update_content(db, state, content).await
        }
        (Key(KeyEvent::E) | Notebook(EditNote), EditingViewMode) => note::edit(state).await,
        (Key(KeyEvent::B) | Notebook(BrowseNoteTree), EditingViewMode) => note::browse(state).await,
        (Key(KeyEvent::Esc) | Notebook(ViewNote), EditingEditMode) => note::view(state).await,
        (Cancel, NoteMoreActions) => {
            let note = state.get_selected_note()?.clone();

            note::select(state, note.clone())
        }
        (Cancel, DirectoryMoreActions) => {
            let directory = state.get_selected_directory()?.clone();

            directory::select(state, directory)
        }
        (Key(KeyEvent::Num(n)), NoteSelected | DirectorySelected) => {
            state.inner_state = NoteTreeNumber(n.into());

            Ok(NotebookTransition::None)
        }
        (Key(KeyEvent::Num(n2)), NoteTreeNumber(n)) => {
            state.inner_state = NoteTreeNumber(n2 + n * 10);

            Ok(NotebookTransition::None)
        }
        (Key(KeyEvent::Esc), NoteTreeNumber(_)) => {
            match state.selected {
                SelectedItem::Note { .. } => {
                    state.inner_state = NoteSelected;
                }
                SelectedItem::Directory { .. } => {
                    state.inner_state = DirectorySelected;
                }
                SelectedItem::None => {}
            };

            Ok(NotebookTransition::None)
        }
        (Key(KeyEvent::J), NoteTreeNumber(n)) => Ok(NotebookTransition::SelectNext(*n)),
        (Key(KeyEvent::K), NoteTreeNumber(n)) => Ok(NotebookTransition::SelectPrev(*n)),
        (event @ Key(_), _) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
