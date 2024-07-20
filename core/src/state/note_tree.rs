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
    browsing_state: BrowsingState,
    note: Note,
    content: String,
}

impl From<EditingState> for InnerState {
    fn from(state: EditingState) -> Self {
        InnerState::Editing(state)
    }
}

#[derive(Clone)]
pub enum BrowsingState {
    NoteSelected(Note),
    NoteMoreActions(Note),

    DirectorySelected(Directory),
    DirectoryMoreActions(Directory),
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

        Ok(NoteTreeState {
            inner_state: Browsing(DirectorySelected(root.directory.clone())),
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
            Browsing(NoteSelected(Note { name, .. })) => format!("Note '{name}' selected"),
            Browsing(DirectorySelected(Directory { name, .. })) => {
                format!("Directory '{name}' selected")
            }
            Browsing(NoteMoreActions(_)) => "Note actions dialog".to_owned(),
            Browsing(DirectoryMoreActions(_)) => "Directory actions dialog".to_owned(),
            Editing(EditingState { mode: EditingMode::View, .. }) => "editing - view".to_owned(),
            Editing(EditingState { mode: EditingMode::Edit, .. }) => "editing - edit".to_owned(),
        }
    }

    pub fn shortcuts(&self) -> Vec<String> {
        match &self.inner_state {
            Browsing(NoteSelected(_)) | Browsing(DirectorySelected { .. }) => {
                vec!["[M] More actions".to_owned()]
            }
            Editing(EditingState { mode: EditingMode::View, .. }) => {
                vec!["[E] Edit mode".to_owned()]
            }
            Editing(EditingState { mode: EditingMode::Edit, .. }) => {
                vec!["[Esc] View mode".to_owned()]
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
            Browsing(DirectorySelected(_)) | Browsing(NoteSelected(_)),
        ) => directory::open(db, state, directory_id).await,
        (
            Event::CloseDirectory(directory_id),
            Browsing(DirectorySelected(_)) | Browsing(NoteSelected(_)),
        ) => directory::close(state, directory_id),
        (Event::Key(KeyEvent::M), Browsing(NoteSelected(ref note))) => {
            note::show_actions_dialog(state, note.clone())
        }
        (Event::Key(KeyEvent::M), Browsing(DirectorySelected(ref directory))) => {
            directory::show_actions_dialog(state, directory.clone())
        }
        (Event::CloseNoteActionsDialog, Browsing(NoteMoreActions(ref note))) => {
            note::select(state, note.clone())
        }
        (Event::CloseDirectoryActionsDialog, Browsing(DirectoryMoreActions(ref directory))) => {
            directory::select(state, directory.clone())
        }
        (Event::SelectNote(note),
            Browsing(DirectorySelected(_)) | Browsing(NoteSelected(_)) | Editing(EditingState { mode: EditingMode::View, .. })
        ) => {
            note::select(state, note)
        }
        (
            Event::SelectDirectory(directory),
            Browsing(DirectorySelected(_)) | Browsing(NoteSelected(_)) | Editing(EditingState { mode: EditingMode::View, .. }),
        ) => directory::select(state, directory),
        (Event::RenameNote(new_name), Browsing(NoteMoreActions(ref note))) => {
            note::rename(db, state, note.clone(), new_name).await
        }
        (Event::RemoveNote, Browsing(NoteMoreActions(ref note))) => {
            note::remove(db, state, note.clone()).await
        }
        (Event::RenameDirectory(new_name), Browsing(DirectoryMoreActions(ref directory))) => {
            directory::rename(db, state, directory.clone(), new_name).await
        }
        (Event::RemoveDirectory, Browsing(DirectoryMoreActions(ref directory))) => {
            directory::remove(db, state, directory.clone()).await
        }
        (Event::AddNote(note_name), Browsing(DirectoryMoreActions(ref directory))) => {
            note::add(db, state, directory.clone(), note_name).await
        }
        (Event::AddDirectory(directory_name), Browsing(DirectoryMoreActions(ref directory))) => {
            directory::add(db, state, directory.clone(), directory_name).await
        }
        (Event::OpenNote, Browsing(s @ NoteSelected(ref note))) => {
            note::open(db, state, s.clone(), note.clone()).await
        }
        (Event::Key(KeyEvent::E), Editing(s @ EditingState { mode: EditingMode::View, .. })) => {
            note::edit(state, s.clone()).await
        }
        (Event::Key(KeyEvent::Esc), Editing(s @ EditingState {
            mode: EditingMode::Edit,
            ..
        })) => {
            note::view(state, s.clone()).await
        }
        (Event::Cancel, Browsing(NoteMoreActions(ref note))) => note::select(state, note.clone()),
        (Event::Cancel, Browsing(DirectoryMoreActions(ref directory))) => {
            directory::select(state, directory.clone())
        }
        (event @ Event::Key(_), _) => Ok(Transition::Inedible(event)),
        _ => Err(Error::Wip("todo: NoteTree::consume".to_owned())),
    }
}
