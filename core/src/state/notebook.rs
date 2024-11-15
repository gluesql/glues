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
    consume::{directory, note, tabs, traverse},
};

pub use inner_state::{
    InnerState::{self, *},
    VimNormalState, VimVisualState,
};

pub use directory_item::{DirectoryItem, DirectoryItemChildren, TreeItem};

pub struct NotebookState {
    pub root: DirectoryItem,
    pub selected: SelectedItem,
    pub tabs: Vec<Note>,
    pub tab_index: Option<usize>,

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
            EditingNormalMode(VimNormalState::Idle) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode")
            }
            EditingNormalMode(VimNormalState::Toggle) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode - toggle")
            }
            EditingNormalMode(VimNormalState::Numbering(n)) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode, steps: '{n}'")
            }
            EditingNormalMode(VimNormalState::Gateway) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode - gateway")
            }
            EditingNormalMode(VimNormalState::Yank(n)) => {
                let name = &self.get_selected_note()?.name;

                let n = if *n >= 2 {
                    format!("{n}")
                } else {
                    "".to_owned()
                };
                format!("Note '{name}' normal mode - yank '{n}y'")
            }
            EditingNormalMode(VimNormalState::Yank2(n1, n2)) => {
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
            EditingNormalMode(VimNormalState::Delete(n)) => {
                let name = &self.get_selected_note()?.name;

                let n = if *n >= 2 {
                    format!("{n}")
                } else {
                    "".to_owned()
                };
                format!("Note '{name}' normal mode - delete '{n}d'")
            }
            EditingNormalMode(VimNormalState::Delete2(n1, n2)) => {
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
            EditingNormalMode(VimNormalState::DeleteInside(n)) => {
                let name = &self.get_selected_note()?.name;
                let n = if *n >= 2 {
                    format!("{n}")
                } else {
                    "".to_owned()
                };

                format!("Note '{name}' normal mode - delete inside {n}di")
            }
            EditingNormalMode(VimNormalState::Change(n)) => {
                let name = &self.get_selected_note()?.name;

                let n = if *n >= 2 {
                    format!("{n}")
                } else {
                    "".to_owned()
                };
                format!("Note '{name}' normal mode - change '{n}c'")
            }
            EditingNormalMode(VimNormalState::Change2(n1, n2)) => {
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
            EditingNormalMode(VimNormalState::ChangeInside(n)) => {
                let name = &self.get_selected_note()?.name;
                let n = if *n >= 2 {
                    format!("{n}")
                } else {
                    "".to_owned()
                };

                format!("Note '{name}' normal mode - change inside {n}ci")
            }
            EditingVisualMode(VimVisualState::Idle) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' visual mode")
            }
            EditingVisualMode(VimVisualState::Numbering(n)) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' visual mode, input: '{n}'")
            }
            EditingVisualMode(VimVisualState::Gateway) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' visual mode - gateway")
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
                    "[l] Open note".to_owned(),
                    "[h] Close parent".to_owned(),
                    "[j|k] Down | Up".to_owned(),
                    "[1-9] Set steps".to_owned(),
                    "[m] More actions".to_owned(),
                    "[Esc] Quit".to_owned(),
                ]
            }
            DirectorySelected => {
                vec![
                    "[l] Toggle".to_owned(),
                    "[h] Close parent".to_owned(),
                    "[j|k] Down | Up".to_owned(),
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
            EditingNormalMode(VimNormalState::Idle) => {
                /*
                    h j k l w e b [1-9] o O 0 $
                    a, A, I, G, g, s, S, x, ^, y, d, u, Ctrl+r
                */
                vec![
                    "[n] Browse notes".to_owned(),
                    "[t] Toggle | Tabs".to_owned(),
                    "[i] Insert".to_owned(),
                    "[v] Visual".to_owned(),
                    "[c] Change".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc] Quit".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Toggle) => {
                vec![
                    "[h|l] Prev | Next Tab".to_owned(),
                    "[x] Close".to_owned(),
                    "[b] Toggle browser".to_owned(),
                    "[n] Toggle line number".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Numbering(n)) => {
                // h j k l [0-9] s S x y d w e b G
                vec![
                    format!("[h|j|k|l] Move cursor {n} steps"),
                    "[0-9] Append steps".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Gateway) => {
                vec![
                    "[g] Move cursor to top".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Yank(n)) => {
                vec![
                    format!("[y] Yank {n} lines"),
                    "[1-9] Append steps".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Yank2(n1, n2)) => {
                vec![
                    if *n1 == 1 {
                        format!("[y] Yank {n2} lines")
                    } else {
                        format!("[y] Yank {n1}*{n2} lines")
                    },
                    "[0-9] Append steps".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Delete(n)) => {
                vec![
                    format!("[i] Inside mode"),
                    format!("[d] Delete {n} lines"),
                    "[0] Delete to line start".to_owned(),
                    "[$] Delete to line end".to_owned(),
                    "[1-9] Append steps".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Delete2(n1, n2)) => {
                vec![
                    if *n1 == 1 {
                        format!("[d] Delete {n2} lines")
                    } else {
                        format!("[d] Delete {n1}*{n2} lines")
                    },
                    "[i] Inside mode".to_owned(),
                    "[0-9] Append steps".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::DeleteInside(n)) => {
                vec![
                    if *n == 1 {
                        "[w] Delete the current word".to_owned()
                    } else {
                        format!("[w] Delete {n} words from cursor")
                    },
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Change(n)) => {
                vec![
                    "[i] Inside mode".to_owned(),
                    format!("[c] Delete {n} lines and insert mode"),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Change2(n1, n2)) => {
                vec![
                    if *n1 == 1 {
                        format!("[c] Delete {n2} lines and insert mode")
                    } else {
                        format!("[c] Delete {n1}*{n2} lines and insert mode")
                    },
                    "[i] Inside mode".to_owned(),
                    "[0-9] Append steps".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::ChangeInside(n)) => {
                vec![
                    if *n == 1 {
                        "[w] Delete the current word and insert mode".to_owned()
                    } else {
                        format!("[w] Delete {n} words from cursor and insert mode")
                    },
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingVisualMode(VimVisualState::Idle) => {
                // more in the keymap
                vec![
                    "[h|j|k|l] Move cursor".to_owned(),
                    "[1-9] Append steps".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingVisualMode(VimVisualState::Numbering(n)) => {
                // more in the keymap
                vec![
                    format!("[h|j|k|l] move cursor {n} steps"),
                    "[0-9] append steps".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingVisualMode(VimVisualState::Gateway) => {
                vec![
                    "[g] Move cursor to top".to_owned(),
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
        let i = self
            .tab_index
            .ok_or_else(|| Error::Wip("tab index is none".to_owned()))?;
        self.tabs
            .get(i)
            .ok_or_else(|| Error::Wip("tab not found".to_owned()))
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
