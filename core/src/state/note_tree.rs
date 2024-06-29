use crate::{
    data::{Directory, Note},
    event::Event,
    types::DirectoryId,
    Error, Glues, Result,
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
        if &self.root.directory.id == directory_id {
            return true;
        }

        let children = match &self.root.children {
            Some(ref children) => children,
            None => return false,
        };

        children
            .directories
            .iter()
            .any(|item| &item.directory.id == directory_id)
    }

    pub async fn consume(_glues: &mut Glues, event: Event) -> Result<()> {
        // let db = &mut glues.db;
        // let state: &mut NoteTreeState = glues.state.get_inner_mut()?;

        match event {
            _ => return Err(Error::Wip("todo: NoteTree::consume".to_owned())),
        };

        /*
            match (&mut glues.state, event) {
                (State::NoteTreeState(ref mut state), Event::SelectNote(note)) => {
                    state.selected = Selected::Note(note);
                }
                (State::NoteTreeState(ref mut state), Event::SelectDirectory(directory_item)) => {
                    state.selected = Selected::Directory(directory_item);
                }
                /*
                (State::NoteTree(ref mut state), Event::OpenDirectory(directory)) => {
                    let item = match state.find_directory_item(&directory.id) {
                        Some(item) => item,
                        None => {
                            return Err(Error::Wip("todo: asdfasdf".to_owned()));
                        }
                    };

                    if item.children.is_none() {
                        let notes = db.fetch_notes(directory.id.clone()).await?;
                        let directories = db
                            .fetch_directories(directory.id.clone())
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

                    Ok(Transition::OpenDirectory {
                        notes: notes.as_slice(),
                        directories: directories.as_slice(),
                    })
                }
                (State::NoteTree(_), Event::CloseDirectory(_directory_item)) => {
                    Ok(Transition::CloseDirectory)
                }
                */
                _ => return Err(Error::Wip("todo: NoteTree::consume".to_owned())),
            };
        */
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

    fn _find_directory_item(&mut self, directory_id: &DirectoryId) -> Option<&mut DirectoryItem> {
        if &self.root.directory.id == directory_id {
            return Some(&mut self.root);
        }

        let children = match &mut self.root.children {
            Some(ref mut children) => children,
            None => return None,
        };

        children
            .directories
            .iter_mut()
            .find(|item| &item.directory.id == directory_id)
    }
}
