use crate::{
    data::{Directory, Note},
    state::GetInner,
    transition::OpenDirectory,
    types::DirectoryId,
    Error, Event, Glues, Result, Transition,
};

pub struct NoteTreeState {
    pub selected: Selected,
    pub root: DirectoryItem,
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

pub enum Selected {
    Note(Note),
    Directory(DirectoryItem),
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
            selected: Selected::Directory(root.clone()),
            root,
        })
    }

    pub fn select_note(&mut self, note: Note) {
        self.selected = Selected::Note(note);
    }

    pub fn select_directory(&mut self, directory_item: DirectoryItem) {
        self.selected = Selected::Directory(directory_item);
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

        match event {
            Event::OpenDirectory(directory_id) => {
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

                Ok(OpenDirectory {
                    notes: notes.as_slice(),
                    directories: directories.as_slice(),
                }
                .into())
            }
            Event::CloseDirectory(directory_id) => {
                state
                    .root
                    .find_mut(&directory_id)
                    .ok_or(Error::Wip("todo: asdfasdf".to_owned()))?
                    .children = None;

                Ok(Transition::CloseDirectory)
            }
            _ => Err(Error::Wip("todo: NoteTree::consume".to_owned())),
        }
    }

    pub fn describe(&self) -> String {
        match &self.selected {
            Selected::Note(Note { name, .. }) => format!("Note '{name}' selected"),
            Selected::Directory(DirectoryItem {
                directory: Directory { name, .. },
                ..
            }) => format!("Directory '{name}' selected"),
        }
    }
}
