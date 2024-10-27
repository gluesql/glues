mod consume;
mod directory_item;
mod inner_state;

use {
    crate::{
        data::{Directory, Note},
        state::GetInner,
        types::DirectoryId,
        Error, Event, Glues, NotebookTransition, Result,
    },
    consume::{directory, note, traverse},
    inner_state::{
        InnerState::{self, *},
        VimState,
    },
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
            EditingNormalMode(VimState::Idle) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode")
            }
            EditingNormalMode(VimState::Numbering(n)) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode, steps: '{n}'")
            }
            EditingInsertMode => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' insert mode")
            }
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
            EditingNormalMode(VimState::Idle) => {
                /* TODO:
                    [o] insert new line below
                    [O] insert new line above
                */

                vec![
                    "[i] Insert mode".to_owned(),
                    "[n] Browse".to_owned(),
                    "[h|j|k|l] Move cursor".to_owned(),
                    "[w|e|b] Word forward|end|back".to_owned(),
                    "[1-9] Set steps".to_owned(),
                    "[t] Toggle line number".to_owned(),
                    "[Esc] Quit".to_owned(),
                ]
            }
            EditingNormalMode(VimState::Numbering(n)) => {
                vec![
                    format!("[h|j|k|l] Move cursor {n} steps"),
                    format!("[w|e|b] Word forward|end|back {n} steps"),
                    "[0-9] Append steps".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingInsertMode => {
                vec![
                    "[Esc] Save note & Normal mode".to_owned(),
                    "[Ctrl+h] Show editor keymap".to_owned(),
                ]
            }
            DirectoryMoreActions | NoteMoreActions => {
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
    let db = glues
        .db
        .as_mut()
        .ok_or(Error::Wip("[consume] empty db".to_owned()))?;
    let state: &mut NotebookState = glues.state.get_inner_mut()?;

    inner_state::consume(db, state, event).await
}
