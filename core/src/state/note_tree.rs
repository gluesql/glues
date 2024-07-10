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

    pub inner_state: InnerState,
}

pub enum InnerState {
    NoteSelected(Note),
    NoteMoreActions(Note),

    DirectorySelected(Directory),
    DirectoryMoreActions(Directory),
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

        Ok(NoteTreeState {
            inner_state: InnerState::DirectorySelected(root.directory.clone()),
            root,
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

    pub fn describe(&self) -> String {
        match &self.inner_state {
            InnerState::NoteSelected(Note { name, .. }) => format!("Note '{name}' selected"),
            InnerState::DirectorySelected(Directory { name, .. }) => {
                format!("Directory '{name}' selected")
            }
            InnerState::NoteMoreActions(_) => "Note actions dialog".to_owned(),
            InnerState::DirectoryMoreActions(_) => "Directory actions dialog".to_owned(),
        }
    }

    pub fn shortcuts(&self) -> Vec<String> {
        match &self.inner_state {
            InnerState::NoteSelected(_) | InnerState::DirectorySelected { .. } => {
                vec!["[M] More actions".to_owned()]
            }
            _ => vec![],
        }
    }
}

pub async fn consume(glues: &mut Glues, event: Event) -> Result<Transition> {
    let db = &mut glues.db;
    let state: &mut NoteTreeState = glues.state.get_inner_mut()?;

    match (event, &state.inner_state) {
        (
            Event::OpenDirectory(directory_id),
            InnerState::DirectorySelected(_) | InnerState::NoteSelected(_),
        ) => directory::open(db, state, directory_id).await,
        (
            Event::CloseDirectory(directory_id),
            InnerState::DirectorySelected(_) | InnerState::NoteSelected(_),
        ) => directory::close(state, directory_id),
        (Event::Key(KeyEvent::M), InnerState::NoteSelected(ref note)) => {
            note::show_actions_dialog(state, note.clone())
        }
        (Event::Key(KeyEvent::M), InnerState::DirectorySelected(ref directory)) => {
            directory::show_actions_dialog(state, directory.clone())
        }
        (Event::CloseNoteActionsDialog, InnerState::NoteMoreActions(ref note)) => {
            note::select(state, note.clone())
        }
        (Event::CloseDirectoryActionsDialog, InnerState::DirectoryMoreActions(ref directory)) => {
            directory::select(state, directory.clone())
        }
        (
            Event::SelectNote(note),
            InnerState::DirectorySelected(_) | InnerState::NoteSelected(_),
        ) => note::select(state, note),
        (
            Event::SelectDirectory(directory),
            InnerState::DirectorySelected(_) | InnerState::NoteSelected(_),
        ) => directory::select(state, directory),
        (Event::RenameNote(new_name), InnerState::NoteMoreActions(ref note)) => {
            note::rename(db, state, note.clone(), new_name).await
        }
        (Event::RemoveNote, InnerState::NoteMoreActions(ref note)) => {
            note::remove(db, state, note.clone()).await
        }
        (Event::RenameDirectory(new_name), InnerState::DirectoryMoreActions(ref directory)) => {
            directory::rename(db, state, directory.clone(), new_name).await
        }
        (Event::RemoveDirectory, InnerState::DirectoryMoreActions(ref directory)) => {
            directory::remove(db, state, directory.clone()).await
        }
        (Event::AddNote(note_name), InnerState::DirectoryMoreActions(ref directory)) => {
            note::add(db, state, directory.clone(), note_name).await
        }
        (Event::AddDirectory(directory_name), InnerState::DirectoryMoreActions(ref directory)) => {
            directory::add(db, state, directory.clone(), directory_name).await
        }
        (Event::Cancel, InnerState::NoteMoreActions(ref note)) => note::select(state, note.clone()),
        (Event::Cancel, InnerState::DirectoryMoreActions(ref directory)) => {
            directory::select(state, directory.clone())
        }
        (Event::Key(_), _) => Ok(Transition::None),
        _ => Err(Error::Wip("todo: NoteTree::consume".to_owned())),
    }
}
