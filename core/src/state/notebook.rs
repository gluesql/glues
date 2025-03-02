mod consume;
mod directory_item;
mod inner_state;

use {
    crate::{
        Error, Event, Glues, NotebookTransition, Result,
        data::{Directory, Note},
        state::GetInner,
        types::{DirectoryId, Id},
    },
    consume::{directory, note, tabs},
};

pub use inner_state::{
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
            EditingNormalMode(VimNormalState::Idle) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode")
            }
            EditingNormalMode(VimNormalState::Toggle) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode - toggle")
            }
            EditingNormalMode(VimNormalState::ToggleTabClose) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode - toggle tab close")
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
            EditingNormalMode(VimNormalState::Scroll) => {
                let name = &self.get_selected_note()?.name;

                format!("Note '{name}' normal mode - scroll")
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
            NoteTree(NoteTreeState::NoteSelected) => {
                let mut shortcuts = vec![
                    "[l]     Open note".to_owned(),
                    "[h]     Close parent directory".to_owned(),
                    "[j]     Select next".to_owned(),
                    "[k]     Select previous".to_owned(),
                    "[G]     Select last".to_owned(),
                    "[g]     Enter gateway mode".to_owned(),
                    "[1-9]   Add steps".to_owned(),
                    "[>]     Expand width".to_owned(),
                    "[<]     Shrink width".to_owned(),
                    "[Space] Move note".to_owned(),
                    "[m]     Show more actions".to_owned(),
                ];

                if !self.tabs.is_empty() {
                    shortcuts.push("[Tab]   Focus editor".to_owned());
                }

                shortcuts.push("[Esc]   Quit".to_owned());
                shortcuts
            }
            NoteTree(NoteTreeState::DirectorySelected) => {
                let mut shortcuts = vec![
                    "[l]     Toggle directory".to_owned(),
                    "[h]     Close parent directory".to_owned(),
                    "[j]     Select next".to_owned(),
                    "[k]     Select previous".to_owned(),
                    "[G]     Select last".to_owned(),
                    "[1-9]   Add steps".to_owned(),
                    "[>]     Expand width".to_owned(),
                    "[<]     Shrink width".to_owned(),
                    "[Space] Move directory".to_owned(),
                    "[m]     Show more actions".to_owned(),
                ];

                if !self.tabs.is_empty() {
                    shortcuts.push("[Tab]   Focus editor".to_owned());
                }

                shortcuts.push("[Esc]   Quit".to_owned());
                shortcuts
            }
            NoteTree(NoteTreeState::Numbering(n)) => {
                vec![
                    format!("[j]   Select {n} next"),
                    format!("[k]   Select {n} previous"),
                    "[G]   Select last".to_owned(),
                    "[0-9] Append steps".to_owned(),
                    format!("[>]   Expand width by {n}"),
                    format!("[<]   Shrink width by {n}"),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            NoteTree(NoteTreeState::GatewayMode) => {
                vec!["[g]   Select first".to_owned(), "[Esc] Cancel".to_owned()]
            }
            NoteTree(NoteTreeState::MoveMode) => {
                vec![
                    "[j]     Select next".to_owned(),
                    "[k]     Select previous".to_owned(),
                    "[G]     Select last".to_owned(),
                    "[Enter] Move to selected directory".to_owned(),
                    "[Esc]   Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Idle) => {
                /*
                    h j k l w e b [1-9] o O 0 $
                    a, A, I, G, g, s, S, x, ^, y, d, u, Ctrl+r
                */
                vec![
                    "[Tab]    Browse notes".to_owned(),
                    "[t]      Enter toggle-tabs mode".to_owned(),
                    "[i]      Enter insert mode".to_owned(),
                    "[v]      Enter visual mode".to_owned(),
                    "[z]      Enter scroll mode".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc]    Quit".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Toggle) => {
                vec![
                    "[h]   select left tab".to_owned(),
                    "[l]   select right tab".to_owned(),
                    "[H]   Move current tab to left".to_owned(),
                    "[L]   Move current tab to right".to_owned(),
                    "[x]   Close current tab".to_owned(),
                    "[X]   Enter tab close mode".to_owned(),
                    "[b]   Toggle browser".to_owned(),
                    "[n]   Toggle editor line number".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::ToggleTabClose) => {
                vec![
                    "[h]   Close left tabs".to_owned(),
                    "[l]   Close right tabs".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Numbering(n)) => {
                // h j k l [0-9] s S x y d w e b G
                vec![
                    format!("[j]      Move cursor {n} steps down"),
                    format!("[k]      Move cursor {n} steps up"),
                    format!("[h]      Move cursor {n} steps left"),
                    format!("[l]      Move cursor {n} steps right"),
                    "[0-9]    Append steps".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc]    Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Gateway) => {
                vec![
                    "[g]   Move cursor to top".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Yank(n)) => {
                vec![
                    format!("[y]   Yank {n} lines"),
                    "[1-9] Append steps".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Yank2(n1, n2)) => {
                vec![
                    if *n1 == 1 {
                        format!("[y]   Yank {n2} lines")
                    } else {
                        format!("[y]   Yank {n1}*{n2} lines")
                    },
                    "[0-9] Append steps".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Delete(n)) => {
                vec![
                    format!("[i]      Enter delete inside mode"),
                    format!("[d]      Delete {n} lines"),
                    "[1-9]    Append steps".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc]    Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Delete2(n1, n2)) => {
                vec![
                    if *n1 == 1 {
                        format!("[d]      Delete {n2} lines")
                    } else {
                        format!("[d]      Delete {n1}*{n2} lines")
                    },
                    "[i]      Enter delete inside mode".to_owned(),
                    "[0-9]    Append steps".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc]    Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::DeleteInside(n)) => {
                vec![
                    if *n == 1 {
                        "[w]   Delete the current word".to_owned()
                    } else {
                        format!("[w]   Delete {n} words from cursor")
                    },
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Change(n)) => {
                vec![
                    "[i]      Enter change inside mode".to_owned(),
                    format!("[c]      Delete {n} lines and enter insert mode"),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc]    Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Change2(n1, n2)) => {
                vec![
                    if *n1 == 1 {
                        format!("[c]      Delete {n2} lines and enter insert mode")
                    } else {
                        format!("[c]      Delete {n1}*{n2} lines and enter insert mode")
                    },
                    "[i]      Enter change inside mode".to_owned(),
                    "[0-9]    Append steps".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc]    Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::ChangeInside(n)) => {
                vec![
                    if *n == 1 {
                        "[w]   Delete the current word and enter insert mode".to_owned()
                    } else {
                        format!("[w]   Delete {n} words from cursor and enter insert mode")
                    },
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingNormalMode(VimNormalState::Scroll) => {
                vec![
                    "[z|.]     Scroll to center".to_owned(),
                    "[t|Enter] Scroll to top".to_owned(),
                    "[b|-]     Scroll to bottom".to_owned(),
                    "[Esc]     Cancel".to_owned(),
                ]
            }
            EditingVisualMode(VimVisualState::Idle) => {
                // more in the keymap
                vec![
                    "[j]      Move cursor down".to_owned(),
                    "[k]      Move cursor up".to_owned(),
                    "[h]      Move cursor left".to_owned(),
                    "[l]      Move cursor right".to_owned(),
                    "[1-9]    Append steps".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc]    Cancel".to_owned(),
                ]
            }
            EditingVisualMode(VimVisualState::Numbering(n)) => {
                // more in the keymap
                vec![
                    format!("[j]      Move cursor {n} steps down"),
                    format!("[k]      Move cursor {n} steps up"),
                    format!("[h]      Move cursor {n} steps left"),
                    format!("[l]      Move cursor {n} steps right"),
                    "[0-9]    Append steps".to_owned(),
                    "[Ctrl+h] Show Vim keymap".to_owned(),
                    "[Esc]    Cancel".to_owned(),
                ]
            }
            EditingVisualMode(VimVisualState::Gateway) => {
                vec![
                    "[g]   Move cursor to top".to_owned(),
                    "[Esc] Cancel".to_owned(),
                ]
            }
            EditingInsertMode => {
                vec![
                    "[Esc]    Save note and enter normal mode".to_owned(),
                    "[Ctrl+h] Show editor keymap".to_owned(),
                ]
            }
            NoteTree(NoteTreeState::DirectoryMoreActions | NoteTreeState::NoteMoreActions) => {
                vec![
                    "[j]     Select next".to_owned(),
                    "[k]     Select Previous".to_owned(),
                    "[Enter] Run selected item".to_owned(),
                    "[Esc]   Close".to_owned(),
                ]
            }
        }
    }

    pub fn get_selected_note(&self) -> Result<&Note> {
        match &self.selected {
            SelectedItem::Note(note) => Ok(note),
            _ => Err(Error::Wip("selected note not found".to_owned())),
        }
    }

    pub fn get_selected_directory(&self) -> Result<&Directory> {
        match &self.selected {
            SelectedItem::Directory(directory) => Ok(directory),
            _ => Err(Error::Wip("selected directory not found".to_owned())),
        }
    }

    pub fn get_selected_id(&self) -> Result<&Id> {
        match &self.selected {
            SelectedItem::Note(note) => Ok(&note.id),
            SelectedItem::Directory(directory) => Ok(&directory.id),
            _ => Err(Error::Wip("selected item not found".to_owned())),
        }
    }

    pub fn get_editing(&self) -> Result<&Note> {
        let i = self
            .tab_index
            .ok_or_else(|| Error::Wip("tab index is none".to_owned()))?;
        self.tabs
            .get(i)
            .map(|tab| &tab.note)
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
