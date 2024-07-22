mod directory;
mod note;

use crate::{
    data::{Directory, Note},
    event::KeyEvent,
    state::GetInner,
    types::DirectoryId,
    Error, Event, Glues, Result, Transition,
};

pub struct NoteTreeState {
    pub root: DirectoryItem,
    pub selected: SelectedItem,
    pub editing: Option<Editing>,

    pub inner_state: InnerState,
}

pub struct Editing {
    note: Note,
    content: String,
}

pub enum SelectedItem {
    Note(Note),
    Directory(Directory),
    None,
}

pub enum InnerState {
    Browsing(BrowsingState),
    Editing(EditingState),
}

#[derive(Clone, Copy)]
pub enum EditingMode {
    View,
    Edit,
}

#[derive(Clone)]
pub struct EditingState {
    mode: EditingMode,
}

impl From<EditingState> for InnerState {
    fn from(state: EditingState) -> Self {
        InnerState::Editing(state)
    }
}

#[derive(Clone)]
pub enum BrowsingState {
    NoteSelected,
    NoteMoreActions,

    DirectorySelected,
    DirectoryMoreActions,
}

use {BrowsingState::*, InnerState::*};

impl From<BrowsingState> for InnerState {
    fn from(state: BrowsingState) -> Self {
        InnerState::Browsing(state)
    }
}

#[derive(Clone)]
pub struct DirectoryItem {
    pub directory: Directory,
    pub children: Option<DirectoryItemChildren>,
}

#[derive(Clone)]
pub struct DirectoryItemChildren {
    pub notes: Vec<Note>,
    pub directories: Vec<DirectoryItem>,
}

impl DirectoryItem {
    fn find(&self, id: &DirectoryId) -> Option<&DirectoryItem> {
        if &self.directory.id == id {
            return Some(self);
        }

        self.children
            .as_ref()?
            .directories
            .iter()
            .filter_map(|item| item.find(id))
            .next()
    }

    fn find_mut(&mut self, id: &DirectoryId) -> Option<&mut DirectoryItem> {
        if &self.directory.id == id {
            return Some(self);
        }

        self.children
            .as_mut()?
            .directories
            .iter_mut()
            .filter_map(|item| item.find_mut(id))
            .next()
    }
}

impl NoteTreeState {
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

        Ok(NoteTreeState {
            inner_state: Browsing(DirectorySelected),
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
            Browsing(NoteMoreActions) => "Note actions dialog".to_owned(),
            Browsing(DirectoryMoreActions) => "Directory actions dialog".to_owned(),
            Browsing(NoteSelected) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' selected")
            }
            Browsing(DirectorySelected) => {
                let name = &self.get_selected_directory()?.name;

                format!("Directory '{name}' selected")
            }
            Editing(EditingState {
                mode: EditingMode::View,
            }) => "editing - view".to_owned(),
            Editing(EditingState {
                mode: EditingMode::Edit,
            }) => "editing - edit".to_owned(),
        })
    }

    pub fn shortcuts(&self) -> Vec<String> {
        match &self.inner_state {
            Browsing(NoteSelected) | Browsing(DirectorySelected) => {
                vec!["[M] More actions".to_owned()]
            }
            Editing(EditingState {
                mode: EditingMode::View,
            }) => {
                vec!["[E] Edit mode".to_owned()]
            }
            Editing(EditingState {
                mode: EditingMode::Edit,
            }) => {
                vec!["[Esc] View mode".to_owned()]
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

    pub fn get_editing(&self) -> Result<&Editing> {
        self.editing
            .as_ref()
            .ok_or_else(|| Error::Wip("editing is none".to_owned()))
    }
}

pub async fn consume(glues: &mut Glues, event: Event) -> Result<Transition> {
    let db = &mut glues.db;
    let state: &mut NoteTreeState = glues.state.get_inner_mut()?;

    match (event, &state.inner_state) {
        (
            Event::OpenDirectory(directory_id),
            Browsing(DirectorySelected) | Browsing(NoteSelected),
        ) => directory::open(db, state, directory_id).await,
        (
            Event::CloseDirectory(directory_id),
            Browsing(DirectorySelected) | Browsing(NoteSelected),
        ) => directory::close(state, directory_id),
        (Event::Key(KeyEvent::M), Browsing(NoteSelected)) => {
            let note = state.get_selected_note()?.clone();

            note::show_actions_dialog(state, note)
        }
        (Event::Key(KeyEvent::M), Browsing(DirectorySelected)) => {
            let directory = state.get_selected_directory()?.clone();

            directory::show_actions_dialog(state, directory)
        }
        (Event::CloseNoteActionsDialog, Browsing(NoteMoreActions)) => {
            let note = state.get_selected_note()?.clone();

            note::select(state, note)
        }
        (Event::CloseDirectoryActionsDialog, Browsing(DirectoryMoreActions)) => {
            let directory = state.get_selected_directory()?.clone();

            directory::select(state, directory)
        }
        (
            Event::SelectNote(note),
            Browsing(DirectorySelected)
            | Browsing(NoteSelected)
            | Editing(EditingState {
                mode: EditingMode::View,
            }),
        ) => note::select(state, note),
        (
            Event::SelectDirectory(directory),
            Browsing(DirectorySelected)
            | Browsing(NoteSelected)
            | Editing(EditingState {
                mode: EditingMode::View,
            }),
        ) => directory::select(state, directory),
        (Event::RenameNote(new_name), Browsing(NoteMoreActions)) => {
            let note = state.get_selected_note()?.clone();

            note::rename(db, state, note, new_name).await
        }
        (Event::RemoveNote, Browsing(NoteMoreActions)) => {
            let note = state.get_selected_note()?.clone();

            note::remove(db, state, note).await
        }
        (Event::RenameDirectory(new_name), Browsing(DirectoryMoreActions)) => {
            let directory = state.get_selected_directory()?.clone();

            directory::rename(db, state, directory, new_name).await
        }
        (Event::RemoveDirectory, Browsing(DirectoryMoreActions)) => {
            let directory = state.get_selected_directory()?.clone();

            directory::remove(db, state, directory).await
        }
        (Event::AddNote(note_name), Browsing(DirectoryMoreActions)) => {
            let directory = state.get_selected_directory()?.clone();

            note::add(db, state, directory, note_name).await
        }
        (Event::AddDirectory(directory_name), Browsing(DirectoryMoreActions)) => {
            let directory = state.get_selected_directory()?.clone();

            directory::add(db, state, directory, directory_name).await
        }
        (Event::OpenNote, Browsing(NoteSelected)) => {
            let note = state.get_selected_note()?.clone();

            note::open(db, state, note).await
        }
        (
            Event::Key(KeyEvent::E),
            Editing(
                s @ EditingState {
                    mode: EditingMode::View,
                },
            ),
        ) => note::edit(state, s.clone()).await,
        (
            Event::Key(KeyEvent::Esc),
            Editing(
                s @ EditingState {
                    mode: EditingMode::Edit,
                },
            ),
        ) => note::view(state, s.clone()).await,
        (Event::Cancel, Browsing(NoteMoreActions)) => {
            let note = state.get_selected_note()?.clone();

            note::select(state, note.clone())
        }
        (Event::Cancel, Browsing(DirectoryMoreActions)) => {
            let directory = state.get_selected_directory()?.clone();

            directory::select(state, directory)
        }
        (event @ Event::Key(_), _) => Ok(Transition::Inedible(event)),
        _ => Err(Error::Wip("todo: NoteTree::consume".to_owned())),
    }
}
