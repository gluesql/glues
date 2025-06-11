mod consume;
mod directory_item;
mod inner_state;

use {
    crate::{
        Error, Event, Glues, NotebookTransition, Result,
        data::{Directory, Note},
        state::GetInner,
        types::{DirectoryId, Id, KeymapGroup, KeymapItem},
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

    pub fn keymap(&self) -> Vec<KeymapGroup> {
        match &self.inner_state {
            NoteTree(NoteTreeState::NoteSelected) => {
                let navigation = vec![
                    KeymapItem::new("j", "Select next"),
                    KeymapItem::new("k", "Select previous"),
                    KeymapItem::new("J", "Select next directory"),
                    KeymapItem::new("K", "Select parent directory"),
                    KeymapItem::new("G", "Select last"),
                    KeymapItem::new("1-9", "Add steps"),
                    KeymapItem::new(">", "Expand width"),
                    KeymapItem::new("<", "Shrink width"),
                ];

                let mut actions = vec![
                    KeymapItem::new("l", "Open note"),
                    KeymapItem::new("h", "Close parent directory"),
                    KeymapItem::new("g", "Enter gateway mode"),
                    KeymapItem::new("Space", "Move note"),
                    KeymapItem::new("m", "Show more actions"),
                ];

                if !self.tabs.is_empty() {
                    actions.push(KeymapItem::new("Tab", "Focus editor"));
                }

                actions.push(KeymapItem::new("Esc", "Quit"));

                vec![
                    KeymapGroup::new("Navigation", navigation),
                    KeymapGroup::new("Actions", actions),
                ]
            }
            NoteTree(NoteTreeState::DirectorySelected) => {
                let mut keymap = vec![
                    KeymapItem::new("l", "Toggle directory"),
                    KeymapItem::new("h", "Close parent directory"),
                    KeymapItem::new("j", "Select next"),
                    KeymapItem::new("k", "Select previous"),
                    KeymapItem::new("J", "Select next directory"),
                    KeymapItem::new("K", "Select previous directory"),
                    KeymapItem::new("G", "Select last"),
                    KeymapItem::new("1-9", "Add steps"),
                    KeymapItem::new(">", "Expand width"),
                    KeymapItem::new("<", "Shrink width"),
                    KeymapItem::new("Space", "Move directory"),
                    KeymapItem::new("m", "Show more actions"),
                ];

                if !self.tabs.is_empty() {
                    keymap.push(KeymapItem::new("Tab", "Focus editor"));
                }

                keymap.push(KeymapItem::new("Esc", "Quit"));
                vec![KeymapGroup::new("General", keymap)]
            }
            NoteTree(NoteTreeState::Numbering(n)) => {
                let items = vec![
                    KeymapItem::new("j", format!("Select {n} next")),
                    KeymapItem::new("k", format!("Select {n} previous")),
                    KeymapItem::new("G", "Select last"),
                    KeymapItem::new("0-9", "Append steps"),
                    KeymapItem::new(">", format!("Expand width by {n}")),
                    KeymapItem::new("<", format!("Shrink width by {n}")),
                    KeymapItem::new("Esc", "Cancel"),
                ];
                vec![KeymapGroup::new("General", items)]
            }
            NoteTree(NoteTreeState::GatewayMode) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("g", "Select first"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            NoteTree(NoteTreeState::MoveMode) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("j", "Select next"),
                        KeymapItem::new("k", "Select previous"),
                        KeymapItem::new("G", "Select last"),
                        KeymapItem::new("Enter", "Move to selected directory"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::Idle) => {
                /*
                    h j k l w e b [1-9] o O 0 $
                    a, A, I, G, g, s, S, x, ^, y, d, u, Ctrl+r
                */
                let items = vec![
                    KeymapItem::new("Tab", "Browse notes"),
                    KeymapItem::new("t", "Enter toggle-tabs mode"),
                    KeymapItem::new("i", "Enter insert mode"),
                    KeymapItem::new("v", "Enter visual mode"),
                    KeymapItem::new("z", "Enter scroll mode"),
                    KeymapItem::new("Ctrl+h", "Show Vim keymap"),
                    KeymapItem::new("Esc", "Quit"),
                ];
                vec![KeymapGroup::new("General", items)]
            }
            EditingNormalMode(VimNormalState::Toggle) => {
                vec![
                    KeymapGroup::new(
                        "Tabs",
                        vec![
                            KeymapItem::new("h", "select left tab"),
                            KeymapItem::new("l", "select right tab"),
                            KeymapItem::new("H", "Move current tab to left"),
                            KeymapItem::new("L", "Move current tab to right"),
                            KeymapItem::new("x", "Close current tab"),
                            KeymapItem::new("X", "Enter tab close mode"),
                        ],
                    ),
                    KeymapGroup::new(
                        "Options",
                        vec![
                            KeymapItem::new("b", "Toggle browser"),
                            KeymapItem::new("n", "Toggle editor line number"),
                            KeymapItem::new("Esc", "Cancel"),
                        ],
                    ),
                ]
            }
            EditingNormalMode(VimNormalState::ToggleTabClose) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("h", "Close left tabs"),
                        KeymapItem::new("l", "Close right tabs"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::Numbering(n)) => {
                // h j k l [0-9] s S x y d w e b G
                let items = vec![
                    KeymapItem::new("j", format!("Move cursor {n} steps down")),
                    KeymapItem::new("k", format!("Move cursor {n} steps up")),
                    KeymapItem::new("h", format!("Move cursor {n} steps left")),
                    KeymapItem::new("l", format!("Move cursor {n} steps right")),
                    KeymapItem::new("0-9", "Append steps"),
                    KeymapItem::new("Ctrl+h", "Show Vim keymap"),
                    KeymapItem::new("Esc", "Cancel"),
                ];
                vec![KeymapGroup::new("General", items)]
            }
            EditingNormalMode(VimNormalState::Gateway) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("g", "Move cursor to top"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::Yank(n)) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("y", format!("Yank {n} lines")),
                        KeymapItem::new("1-9", "Append steps"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::Yank2(n1, n2)) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        if *n1 == 1 {
                            KeymapItem::new("y", format!("Yank {n2} lines"))
                        } else {
                            KeymapItem::new("y", format!("Yank {n1}*{n2} lines"))
                        },
                        KeymapItem::new("0-9", "Append steps"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::Delete(n)) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("i", "Enter delete inside mode"),
                        KeymapItem::new("d", format!("Delete {n} lines")),
                        KeymapItem::new("0", "Delete from start of line"),
                        KeymapItem::new("b", "Delete previous word"),
                        KeymapItem::new("e", "Delete to word end"),
                        KeymapItem::new("h", "Delete previous character"),
                        KeymapItem::new("l", "Delete next character"),
                        KeymapItem::new("$", "Delete to line end"),
                        KeymapItem::new("1-9", "Append steps"),
                        KeymapItem::new("Ctrl+h", "Show Vim keymap"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::Delete2(n1, n2)) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        if *n1 == 1 {
                            KeymapItem::new("d", format!("Delete {n2} lines"))
                        } else {
                            KeymapItem::new("d", format!("Delete {n1}*{n2} lines"))
                        },
                        KeymapItem::new("i", "Enter delete inside mode"),
                        KeymapItem::new("b", "Delete previous word"),
                        KeymapItem::new("e", "Delete to word end"),
                        KeymapItem::new("h", "Delete previous character"),
                        KeymapItem::new("l", "Delete next character"),
                        KeymapItem::new("$", "Delete to line end"),
                        KeymapItem::new("0-9", "Append steps"),
                        KeymapItem::new("Ctrl+h", "Show Vim keymap"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::DeleteInside(n)) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        if *n == 1 {
                            KeymapItem::new("w", "Delete the current word")
                        } else {
                            KeymapItem::new("w", format!("Delete {n} words from cursor"))
                        },
                        KeymapItem::new("Ctrl+h", "Show Vim keymap"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::Change(n)) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("i", "Enter change inside mode"),
                        KeymapItem::new("c", format!("Delete {n} lines and enter insert mode")),
                        KeymapItem::new("Ctrl+h", "Show Vim keymap"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::Change2(n1, n2)) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        if *n1 == 1 {
                            KeymapItem::new("c", format!("Delete {n2} lines and enter insert mode"))
                        } else {
                            KeymapItem::new(
                                "c",
                                format!("Delete {n1}*{n2} lines and enter insert mode"),
                            )
                        },
                        KeymapItem::new("i", "Enter change inside mode"),
                        KeymapItem::new("0-9", "Append steps"),
                        KeymapItem::new("Ctrl+h", "Show Vim keymap"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::ChangeInside(n)) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        if *n == 1 {
                            KeymapItem::new("w", "Delete the current word and enter insert mode")
                        } else {
                            KeymapItem::new(
                                "w",
                                format!("Delete {n} words from cursor and enter insert mode"),
                            )
                        },
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingNormalMode(VimNormalState::Scroll) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("z|.", "Scroll to center"),
                        KeymapItem::new("t|Enter", "Scroll to top"),
                        KeymapItem::new("b|-", "Scroll to bottom"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingVisualMode(VimVisualState::Idle) => {
                // more in the keymap
                let items = vec![
                    KeymapItem::new("j", "Move cursor down"),
                    KeymapItem::new("k", "Move cursor up"),
                    KeymapItem::new("h", "Move cursor left"),
                    KeymapItem::new("l", "Move cursor right"),
                    KeymapItem::new("1-9", "Append steps"),
                    KeymapItem::new("Ctrl+h", "Show Vim keymap"),
                    KeymapItem::new("Esc", "Cancel"),
                ];
                vec![KeymapGroup::new("General", items)]
            }
            EditingVisualMode(VimVisualState::Numbering(n)) => {
                // more in the keymap
                let items = vec![
                    KeymapItem::new("j", format!("Move cursor {n} steps down")),
                    KeymapItem::new("k", format!("Move cursor {n} steps up")),
                    KeymapItem::new("h", format!("Move cursor {n} steps left")),
                    KeymapItem::new("l", format!("Move cursor {n} steps right")),
                    KeymapItem::new("0-9", "Append steps"),
                    KeymapItem::new("Ctrl+h", "Show Vim keymap"),
                    KeymapItem::new("Esc", "Cancel"),
                ];
                vec![KeymapGroup::new("General", items)]
            }
            EditingVisualMode(VimVisualState::Gateway) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("g", "Move cursor to top"),
                        KeymapItem::new("Esc", "Cancel"),
                    ],
                )]
            }
            EditingInsertMode => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("Esc", "Save note and enter normal mode"),
                        KeymapItem::new("Ctrl+h", "Show editor keymap"),
                    ],
                )]
            }
            NoteTree(NoteTreeState::DirectoryMoreActions | NoteTreeState::NoteMoreActions) => {
                vec![KeymapGroup::new(
                    "General",
                    vec![
                        KeymapItem::new("j", "Select next"),
                        KeymapItem::new("k", "Select Previous"),
                        KeymapItem::new("Enter", "Run selected item"),
                        KeymapItem::new("Esc", "Close"),
                    ],
                )]
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
