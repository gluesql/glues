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
                vec!["[Enter] Open note", "[M] More actions"]
            }
            DirectorySelected => {
                vec!["[Enter] Toggle", "[M] More actions"]
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

pub async fn consume(glues: &mut Glues, event: Event) -> Result<Transition> {
    let db = &mut glues.db;
    let state: &mut NoteTreeState = glues.state.get_inner_mut()?;

    match (event, &state.inner_state) {
        (Event::OpenDirectory(directory_id), DirectorySelected | NoteSelected) => {
            directory::open(db, state, directory_id).await
        }
        (Event::CloseDirectory(directory_id), DirectorySelected | NoteSelected) => {
            directory::close(state, directory_id)
        }
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
        (Event::OpenNote, NoteSelected) => {
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
        (event @ Event::Key(_), _) => Ok(Transition::Inedible(event)),
        _ => Err(Error::Wip("todo: NoteTree::consume".to_owned())),
    }
}
