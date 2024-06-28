use crate::{
    data::{Directory, Note},
    event::Event,
    state::State,
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
        let root_directory = glues.fetch_directory(glues.root_id.clone()).await?;
        let notes = glues.fetch_notes(root_directory.id.clone()).await?;
        let directories = glues
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

    pub async fn consume(glues: &mut Glues, event: Event) -> Result<()> {
        match (&mut glues.state, event) {
            (State::NoteTree(ref mut state), Event::SelectNote(note)) => {
                state.selected = Selected::Note(note);
            }
            (State::NoteTree(ref mut state), Event::SelectDirectory(directory_item)) => {
                state.selected = Selected::Directory(directory_item);
            }
            _ => return Err(Error::Wip("todo: NoteTree::consume".to_owned())),
        };

        Ok(())
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
