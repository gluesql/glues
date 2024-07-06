use crate::{
    data::{Directory, Note},
    event::KeyEvent,
    state::GetInner,
    transition::{ShowDirectoryActionsDialog, ShowNoteActionsDialog},
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

    DirectorySelected { id: DirectoryId, name: String },
    DirectoryMoreActions { id: DirectoryId, name: String },
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
            inner_state: InnerState::DirectorySelected {
                id: root.directory.id.clone(),
                name: root.directory.name.clone(),
            },
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

    pub async fn consume(glues: &mut Glues, event: Event) -> Result<Transition> {
        let db = &mut glues.db;
        let state: &mut NoteTreeState = glues.state.get_inner_mut()?;

        match (&state.inner_state, event) {
            (
                InnerState::DirectorySelected { .. } | InnerState::NoteSelected(_),
                Event::OpenDirectory(directory_id),
            ) => {
                let item = state
                    .root
                    .find_mut(&directory_id)
                    .ok_or(Error::Wip("todo: asdfasdf".to_owned()))?;

                if item.children.is_none() {
                    let notes = db.fetch_notes(directory_id.clone()).await?;
                    let directories = db
                        .fetch_directories(directory_id.clone())
                        .await?
                        .into_iter()
                        .map(|directory| DirectoryItem {
                            directory,
                            children: None,
                        })
                        .collect();

                    item.children = Some(DirectoryItemChildren { notes, directories });
                }

                let (notes, directories) = match &mut item.children {
                    Some(children) => (&children.notes, &children.directories),
                    None => {
                        panic!("...?");
                    }
                };

                return Ok(Transition::OpenDirectory {
                    id: directory_id.clone(),
                    notes: notes.clone(),
                    directories: directories.clone(),
                });
            }
            (
                InnerState::DirectorySelected { .. } | InnerState::NoteSelected(_),
                Event::CloseDirectory(directory_id),
            ) => {
                state
                    .root
                    .find_mut(&directory_id)
                    .ok_or(Error::Wip("todo: asdfasdf".to_owned()))?
                    .children = None;

                return Ok(Transition::CloseDirectory(directory_id.clone()));
            }
            (InnerState::NoteSelected(ref note), Event::Key(KeyEvent::M)) => {
                let note = note.clone();
                state.inner_state = InnerState::NoteMoreActions(note.clone());

                return Ok(ShowNoteActionsDialog { note }.into());
            }
            (InnerState::DirectorySelected { id, name }, Event::Key(KeyEvent::M)) => {
                let id = id.clone();
                let name = name.clone();

                state.inner_state = InnerState::DirectoryMoreActions {
                    id: id.clone(),
                    name: name.clone(),
                };

                // TODO: only name field should be used
                let directory = Directory {
                    id,
                    name,
                    parent_id: "".to_owned(),
                };

                return Ok(ShowDirectoryActionsDialog { directory }.into());
            }
            (InnerState::NoteMoreActions(ref note), Event::CloseNoteActionsDialog) => {
                state.inner_state = InnerState::NoteSelected(note.clone());
            }
            (InnerState::DirectoryMoreActions { id, name }, Event::CloseDirectoryActionsDialog) => {
                state.inner_state = InnerState::DirectorySelected {
                    id: id.clone(),
                    name: name.clone(),
                };
            }
            (
                InnerState::DirectorySelected { .. } | InnerState::NoteSelected(_),
                Event::SelectNote(note),
            ) => {
                state.inner_state = InnerState::NoteSelected(note.clone());
            }
            (
                InnerState::DirectorySelected { .. } | InnerState::NoteSelected(_),
                Event::SelectDirectory { id, name },
            ) => {
                state.inner_state = InnerState::DirectorySelected { id, name };
            }
            (InnerState::NoteMoreActions(ref note), Event::RenameNote(new_name)) => {
                let id = note.id.clone();
                db.rename_note(id.clone(), new_name.clone()).await?;

                state.inner_state = InnerState::NoteSelected(note.clone());

                return Ok(Transition::RenameNote { id, name: new_name });
            }
            (InnerState::DirectoryMoreActions { id, .. }, Event::RenameDirectory(new_name)) => {
                let id = id.clone();
                db.rename_directory(id.clone(), new_name.clone()).await?;

                state.inner_state = InnerState::DirectorySelected {
                    id: id.clone(),
                    name: new_name.clone(),
                };

                return Ok(Transition::RenameDirectory { id, name: new_name });
            }
            (_, Event::Key(_)) => {}
            _ => return Err(Error::Wip("todo: NoteTree::consume".to_owned())),
        };

        Ok(Transition::None)
    }

    pub fn describe(&self) -> String {
        match &self.inner_state {
            InnerState::NoteSelected(Note { name, .. }) => format!("Note '{name}' selected"),
            InnerState::DirectorySelected { name, .. } => format!("Directory '{name}' selected"),
            InnerState::NoteMoreActions(_) => "Note actions dialog".to_owned(),
            InnerState::DirectoryMoreActions { .. } => "Directory actions dialog".to_owned(),
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
