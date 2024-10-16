use {
    crate::{
        action::{Action, TuiAction},
        logger::*,
    },
    edtui::{EditorEventHandler, EditorMode, EditorState, Lines},
    glues_core::{
        data::{Directory, Note},
        state::notebook::DirectoryItem,
        types::Id,
        NotebookEvent,
    },
    ratatui::{
        crossterm::event::{Event, KeyCode},
        text::Line,
        widgets::ListState,
    },
};

pub const REMOVE_NOTE: &str = "Remove note";
pub const RENAME_NOTE: &str = "Rename note";

pub const ADD_NOTE: &str = "Add note";
pub const ADD_DIRECTORY: &str = "Add directory";
pub const RENAME_DIRECTORY: &str = "Rename directory";
pub const REMOVE_DIRECTORY: &str = "Remove directory";

pub const CLOSE: &str = "Close";

pub const NOTE_ACTIONS: [&str; 3] = [RENAME_NOTE, REMOVE_NOTE, CLOSE];
pub const DIRECTORY_ACTIONS: [&str; 5] = [
    ADD_NOTE,
    ADD_DIRECTORY,
    RENAME_DIRECTORY,
    REMOVE_DIRECTORY,
    CLOSE,
];

#[derive(Clone, Copy, PartialEq)]
pub enum ContextState {
    NoteTreeBrowsing,
    NoteActionsDialog,
    DirectoryActionsDialog,
    EditorViewMode,
    EditorEditMode,
}

pub struct NotebookContext {
    pub state: ContextState,

    // note tree
    pub tree_state: ListState,
    pub tree_items: Vec<TreeItem>,

    // note actions
    pub note_actions_state: ListState,

    // directory actions
    pub directory_actions_state: ListState,

    // editor
    pub editor_state: EditorState,
    pub editor_handler: EditorEventHandler,
    pub opened_note: Option<Note>,
}

impl Default for NotebookContext {
    fn default() -> Self {
        Self {
            state: ContextState::NoteTreeBrowsing,
            tree_state: ListState::default().with_selected(Some(0)),
            tree_items: vec![],

            note_actions_state: ListState::default(),
            directory_actions_state: ListState::default(),

            editor_state: EditorState::new(Lines::from("\n   >\")++++<")),
            editor_handler: EditorEventHandler::default(),
            opened_note: None,
        }
    }
}

impl NotebookContext {
    pub fn update_items(&mut self, directory_item: &DirectoryItem) {
        self.tree_items = flatten(directory_item, 0);
    }

    pub fn select_item(&mut self, id: &Id) {
        for (i, item) in self.tree_items.iter().enumerate() {
            let item_id = match item {
                TreeItem::Directory { value, .. } => &value.id,
                TreeItem::Note { value, .. } => &value.id,
            };

            if item_id == id {
                self.tree_state.select(Some(i));
                break;
            }
        }
    }

    pub fn open_note(&mut self, note: Note, content: String) {
        self.state = ContextState::EditorViewMode;
        self.opened_note = Some(note);
        self.editor_state = EditorState::new(Lines::from(content.as_str()));
    }

    pub fn consume(&mut self, code: KeyCode) -> Action {
        match self.state {
            ContextState::NoteTreeBrowsing => self.consume_on_note_tree(code),
            ContextState::EditorViewMode | ContextState::EditorEditMode => {
                self.consume_on_editor(code)
            }
            ContextState::NoteActionsDialog => self.consume_on_note_actions(code),
            ContextState::DirectoryActionsDialog => self.consume_on_directory_actions(code),
        }
    }

    fn consume_on_note_tree(&mut self, code: KeyCode) -> Action {
        macro_rules! item {
            () => {
                self.tree_state
                    .selected()
                    .and_then(|idx| self.tree_items.get(idx))
                    .log_expect("[NotebookContext::consume] selected must not be empty")
            };
        }

        match code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.tree_state.select_next();

                match self
                    .tree_state
                    .selected()
                    .and_then(|i| self.tree_items.get(i))
                {
                    Some(TreeItem::Directory { value, .. }) => {
                        Action::Dispatch(NotebookEvent::SelectDirectory(value.clone()).into())
                    }
                    Some(TreeItem::Note { value, .. }) => {
                        Action::Dispatch(NotebookEvent::SelectNote(value.clone()).into())
                    }
                    None => {
                        self.tree_state.select_last();
                        Action::None
                    }
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.tree_state.select_previous();

                match item!() {
                    TreeItem::Directory { value, .. } => {
                        Action::Dispatch(NotebookEvent::SelectDirectory(value.clone()).into())
                    }
                    TreeItem::Note { value, .. } => {
                        Action::Dispatch(NotebookEvent::SelectNote(value.clone()).into())
                    }
                }
            }
            KeyCode::Char('l') => match item!() {
                TreeItem::Directory { value, opened, .. } => {
                    let directory_id = value.id.clone();
                    let event = if *opened {
                        NotebookEvent::CloseDirectory(directory_id)
                    } else {
                        NotebookEvent::OpenDirectory(directory_id)
                    };

                    Action::Dispatch(event.into())
                }
                TreeItem::Note { .. } => Action::None,
            },
            KeyCode::Char('m') => match item!() {
                TreeItem::Directory { .. } => {
                    self.state = ContextState::DirectoryActionsDialog;
                    self.directory_actions_state.select_first();

                    Action::PassThrough
                }
                TreeItem::Note { .. } => {
                    self.state = ContextState::NoteActionsDialog;
                    self.note_actions_state.select_first();

                    Action::PassThrough
                }
            },
            KeyCode::Char('o') | KeyCode::Char('b') | KeyCode::Char('e') | KeyCode::Char('h') => {
                Action::PassThrough
            }
            KeyCode::Esc => TuiAction::Confirm {
                message: "Do you want to quit?".to_owned(),
                action: Box::new(TuiAction::Quit.into()),
            }
            .into(),
            _ => Action::None,
        }
    }

    fn consume_on_editor(&mut self, code: KeyCode) -> Action {
        let mode = self.editor_state.mode;

        match code {
            KeyCode::Char('/') | KeyCode::Char('v') if mode == EditorMode::Normal => Action::None,
            KeyCode::Char('b') if mode == EditorMode::Normal => {
                self.state = ContextState::NoteTreeBrowsing;
                self.opened_note = None;

                Action::Dispatch(NotebookEvent::BrowseNoteTree.into())
            }
            KeyCode::Esc if mode == EditorMode::Insert => {
                self.state = ContextState::EditorViewMode;
                self.editor_handler
                    .on_event(Event::Key(code.into()), &mut self.editor_state);

                Action::Dispatch(NotebookEvent::ViewNote.into())
            }
            KeyCode::Esc => TuiAction::Confirm {
                message: "Do you want to quit?".to_owned(),
                action: Box::new(TuiAction::Quit.into()),
            }
            .into(),
            _ => {
                self.editor_handler
                    .on_event(Event::Key(code.into()), &mut self.editor_state);

                let new_mode = self.editor_state.mode;
                if mode != new_mode {
                    match new_mode {
                        EditorMode::Normal => {
                            self.state = ContextState::EditorViewMode;
                            Action::Dispatch(NotebookEvent::ViewNote.into())
                        }
                        EditorMode::Insert => {
                            self.state = ContextState::EditorEditMode;
                            Action::Dispatch(NotebookEvent::EditNote.into())
                        }
                        _ => Action::None,
                    }
                } else {
                    Action::None
                }
            }
        }
    }

    fn consume_on_note_actions(&mut self, code: KeyCode) -> Action {
        match code {
            KeyCode::Char('j') => {
                self.note_actions_state.select_next();
                Action::None
            }
            KeyCode::Char('k') => {
                self.note_actions_state.select_previous();
                Action::None
            }
            KeyCode::Esc => {
                self.state = ContextState::NoteTreeBrowsing;

                Action::Dispatch(NotebookEvent::CloseNoteActionsDialog.into())
            }
            KeyCode::Enter => {
                match NOTE_ACTIONS[self
                    .note_actions_state
                    .selected()
                    .log_expect("note action must not be empty")]
                {
                    RENAME_NOTE => TuiAction::Prompt {
                        message: vec![Line::raw("Enter new note name:")],
                        action: Box::new(TuiAction::RenameNote.into()),
                        default: None,
                    }
                    .into(),
                    REMOVE_NOTE => TuiAction::Confirm {
                        message: "Confirm to remove note?".to_owned(),
                        action: Box::new(TuiAction::RemoveNote.into()),
                    }
                    .into(),
                    CLOSE => {
                        self.state = ContextState::NoteTreeBrowsing;

                        Action::Dispatch(NotebookEvent::CloseNoteActionsDialog.into())
                    }
                    _ => Action::None,
                }
            }
            _ => Action::None,
        }
    }

    fn consume_on_directory_actions(&mut self, code: KeyCode) -> Action {
        match code {
            KeyCode::Char('j') => {
                self.directory_actions_state.select_next();
                Action::None
            }
            KeyCode::Char('k') => {
                self.directory_actions_state.select_previous();
                Action::None
            }
            KeyCode::Enter => {
                match DIRECTORY_ACTIONS[self
                    .directory_actions_state
                    .selected()
                    .log_expect("directory action must not be empty")]
                {
                    ADD_NOTE => TuiAction::Prompt {
                        message: vec![Line::raw("Enter note name:")],
                        action: Box::new(TuiAction::AddNote.into()),
                        default: None,
                    }
                    .into(),
                    ADD_DIRECTORY => TuiAction::Prompt {
                        message: vec![Line::raw("Enter directory name:")],
                        action: Box::new(TuiAction::AddDirectory.into()),
                        default: None,
                    }
                    .into(),
                    RENAME_DIRECTORY => TuiAction::Prompt {
                        message: vec![Line::raw("Enter new directory name:")],
                        action: Box::new(TuiAction::RenameDirectory.into()),
                        default: None,
                    }
                    .into(),
                    REMOVE_DIRECTORY => TuiAction::Confirm {
                        message: "Confirm to remove directory?".to_owned(),
                        action: Box::new(TuiAction::RemoveDirectory.into()),
                    }
                    .into(),
                    CLOSE => {
                        self.state = ContextState::NoteTreeBrowsing;

                        Action::Dispatch(NotebookEvent::CloseDirectoryActionsDialog.into())
                    }
                    _ => Action::None,
                }
            }
            KeyCode::Esc => {
                self.state = ContextState::NoteTreeBrowsing;

                Action::Dispatch(NotebookEvent::CloseDirectoryActionsDialog.into())
            }
            _ => Action::None,
        }
    }
}

#[derive(Clone)]
pub enum TreeItem {
    Note {
        value: Note,
        depth: usize,
    },
    Directory {
        value: Directory,
        depth: usize,
        opened: bool,
    },
}

fn flatten(directory_item: &DirectoryItem, depth: usize) -> Vec<TreeItem> {
    let mut items = vec![TreeItem::Directory {
        value: directory_item.directory.clone(),
        depth,
        opened: directory_item.children.is_some(),
    }];

    if let Some(children) = &directory_item.children {
        for item in &children.directories {
            items.extend(flatten(item, depth + 1));
        }

        for note in &children.notes {
            items.push(TreeItem::Note {
                value: note.clone(),
                depth: depth + 1,
            });
        }
    }

    items
}
