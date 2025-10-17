mod consume;
mod directory_item;
mod inner_state;

use {
    crate::{
        Error, Event, Glues, NotebookTransition, Result,
        data::{Directory, Note},
        state::GetInner,
        types::{DirectoryId, Id, KeymapGroup},
    },
    consume::{directory, note, tabs},
};

pub use inner_state::{
    EditorState,
    InnerState::{self, *},
    NoteTreeState, VimNormalState, VimVisualState,
};

pub use directory_item::{DirectoryItem, DirectoryItemChildren, TreeItem};

pub struct NotebookState {
    pub root: DirectoryItem,
    pub selected: SelectedItem,
    pub tabs: Vec<Tab>,
    pub tab_index: Option<usize>,

    pub inner_state: InnerState,
}

#[derive(Clone)]
pub struct Tab {
    pub note: Note,
    pub breadcrumb: Vec<String>,
}

pub enum SelectedItem {
    Note(Note),
    Directory(Directory),
    None,
}

impl NotebookState {
    pub async fn new(glues: &mut Glues) -> Result<Self> {
        let db = glues.db.as_mut().ok_or(Error::InvalidState(
            "[NotebookState::new] empty db".to_owned(),
        ))?;
        let root_id = db.root_id();
        let root_directory = db.fetch_directory(root_id.clone()).await?;
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
            inner_state: NoteTree(NoteTreeState::DirectorySelected),
            root,
            selected,
            tabs: Vec::new(),
            tab_index: None,
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
            NoteTree(NoteTreeState::NoteMoreActions) => "Note actions dialog".to_owned(),
            NoteTree(NoteTreeState::DirectoryMoreActions) => "Directory actions dialog".to_owned(),
            NoteTree(NoteTreeState::NoteSelected) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' selected")
            }
            NoteTree(NoteTreeState::DirectorySelected) => {
                let name = &self.get_selected_directory()?.name;

                format!("Directory '{name}' selected")
            }
            NoteTree(NoteTreeState::Numbering(n)) => {
                format!("Steps: '{n}' selected")
            }
            NoteTree(NoteTreeState::GatewayMode) => "Gateway mode".to_owned(),
            NoteTree(NoteTreeState::MoveMode) => match &self.selected {
                SelectedItem::Note(Note { name, .. }) => {
                    format!("Note move mode: '{name}'")
                }
                SelectedItem::Directory(Directory { name, .. }) => {
                    format!("Directory move mode: '{name}'")
                }
                _ => "Move mode".to_owned(),
            },
            Editor(EditorState::Normal(VimNormalState::Idle)) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode")
            }
            Editor(EditorState::Normal(VimNormalState::Toggle)) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode - toggle")
            }
            Editor(EditorState::Normal(VimNormalState::ToggleTabClose)) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode - toggle tab close")
            }
            Editor(EditorState::Normal(VimNormalState::Numbering(n))) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode, steps: '{n}'")
            }
            Editor(EditorState::Normal(VimNormalState::Gateway)) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode - gateway")
            }
            Editor(EditorState::Normal(VimNormalState::Yank(n))) => {
                let name = &self.get_selected_note()?.name;

                let n = if *n >= 2 {
                    format!("{n}")
                } else {
                    "".to_owned()
                };
                format!("Note '{name}' normal mode - yank '{n}y'")
            }
            Editor(EditorState::Normal(VimNormalState::Yank2(n1, n2))) => {
                let name = &self.get_selected_note()?.name;
                let n1 = if *n1 >= 2 {
                    format!("{n1}")
                } else {
                    "".to_owned()
                };
                let n2 = if *n2 >= 2 {
                    format!("{n2}")
                } else {
                    "".to_owned()
                };

                format!("Note '{name}' normal mode - yank '{n1}y{n2}'")
            }
            Editor(EditorState::Normal(VimNormalState::Delete(n))) => {
                let name = &self.get_selected_note()?.name;

                let n = if *n >= 2 {
                    format!("{n}")
                } else {
                    "".to_owned()
                };
                format!("Note '{name}' normal mode - delete '{n}d'")
            }
            Editor(EditorState::Normal(VimNormalState::Delete2(n1, n2))) => {
                let name = &self.get_selected_note()?.name;
                let n1 = if *n1 >= 2 {
                    format!("{n1}")
                } else {
                    "".to_owned()
                };
                let n2 = if *n2 >= 2 {
                    format!("{n2}")
                } else {
                    "".to_owned()
                };

                format!("Note '{name}' normal mode - delete '{n1}d{n2}'")
            }
            Editor(EditorState::Normal(VimNormalState::DeleteInside(n))) => {
                let name = &self.get_selected_note()?.name;
                let n = if *n >= 2 {
                    format!("{n}")
                } else {
                    "".to_owned()
                };

                format!("Note '{name}' normal mode - delete inside {n}di")
            }
            Editor(EditorState::Normal(VimNormalState::Change(n))) => {
                let name = &self.get_selected_note()?.name;

                let n = if *n >= 2 {
                    format!("{n}")
                } else {
                    "".to_owned()
                };
                format!("Note '{name}' normal mode - change '{n}c'")
            }
            Editor(EditorState::Normal(VimNormalState::Change2(n1, n2))) => {
                let name = &self.get_selected_note()?.name;
                let n1 = if *n1 >= 2 {
                    format!("{n1}")
                } else {
                    "".to_owned()
                };
                let n2 = if *n2 >= 2 {
                    format!("{n2}")
                } else {
                    "".to_owned()
                };

                format!("Note '{name}' normal mode - change '{n1}c{n2}'")
            }
            Editor(EditorState::Normal(VimNormalState::ChangeInside(n))) => {
                let name = &self.get_selected_note()?.name;
                let n = if *n >= 2 {
                    format!("{n}")
                } else {
                    "".to_owned()
                };

                format!("Note '{name}' normal mode - change inside {n}ci")
            }
            Editor(EditorState::Normal(VimNormalState::Scroll)) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode - scroll")
            }
            Editor(EditorState::Visual(VimVisualState::Idle)) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' visual mode")
            }
            Editor(EditorState::Visual(VimVisualState::Numbering(n))) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' visual mode, input: '{n}'")
            }
            Editor(EditorState::Visual(VimVisualState::Gateway)) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' visual mode - gateway")
            }
            Editor(EditorState::Insert) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' insert mode")
            }
        })
    }

    pub fn keymap(&self) -> Vec<KeymapGroup> {
        inner_state::keymap(self)
    }

    pub fn get_selected_note(&self) -> Result<&Note> {
        match &self.selected {
            SelectedItem::Note(note) => Ok(note),
            _ => Err(Error::InvalidState("selected note not found".to_owned())),
        }
    }

    pub fn get_selected_directory(&self) -> Result<&Directory> {
        match &self.selected {
            SelectedItem::Directory(directory) => Ok(directory),
            _ => Err(Error::InvalidState(
                "selected directory not found".to_owned(),
            )),
        }
    }

    pub fn get_selected_id(&self) -> Result<&Id> {
        match &self.selected {
            SelectedItem::Note(note) => Ok(&note.id),
            SelectedItem::Directory(directory) => Ok(&directory.id),
            _ => Err(Error::InvalidState("selected item not found".to_owned())),
        }
    }

    pub fn get_editing(&self) -> Result<&Note> {
        let i = self
            .tab_index
            .ok_or_else(|| Error::InvalidState("tab index is none".to_owned()))?;
        self.tabs
            .get(i)
            .map(|tab| &tab.note)
            .ok_or_else(|| Error::NotFound("tab not found".to_owned()))
    }
}

pub async fn consume(glues: &mut Glues, event: Event) -> Result<NotebookTransition> {
    let db = glues
        .db
        .as_mut()
        .ok_or(Error::InvalidState("[consume] empty db".to_owned()))?;
    let state: &mut NotebookState = glues.state.get_inner_mut()?;

    inner_state::consume(db, state, event).await
}
