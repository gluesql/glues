use {
    crate::{
        action::{Action, TuiAction},
        logger::*,
    },
    glues_core::{
        data::{Directory, Note},
        state::notebook::DirectoryItem,
        types::{Id, NoteId},
        NotebookEvent,
    },
    ratatui::{
        crossterm::event::{Event as Input, KeyCode, KeyEvent, KeyModifiers},
        text::Line,
        widgets::ListState,
    },
    tui_textarea::TextArea,
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
    NoteTreeNumbering,
    NoteActionsDialog,
    DirectoryActionsDialog,
    EditorNormalMode { idle: bool },
    EditorVisualMode,
    EditorInsertMode,
}

impl ContextState {
    pub fn is_editor(&self) -> bool {
        matches!(
            self,
            ContextState::EditorNormalMode { .. }
                | ContextState::EditorInsertMode
                | ContextState::EditorVisualMode
        )
    }
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
    pub tabs: Vec<EditorTab>,
    pub tab_index: Option<usize>,
    pub show_line_number: bool,
    pub show_browser: bool,
    pub line_yanked: bool,
    pub yank: Option<String>,
}

pub struct EditorTab {
    pub note: Note,
    pub editor: TextArea<'static>,
}

impl Default for NotebookContext {
    fn default() -> Self {
        Self {
            state: ContextState::NoteTreeBrowsing,
            tree_state: ListState::default().with_selected(Some(0)),
            tree_items: vec![],

            note_actions_state: ListState::default(),
            directory_actions_state: ListState::default(),

            tabs: vec![],
            tab_index: None,
            show_line_number: true,
            show_browser: true,
            line_yanked: false,
            yank: None,
        }
    }
}

impl NotebookContext {
    pub fn get_opened_note(&self) -> Option<&Note> {
        self.tab_index
            .and_then(|i| self.tabs.get(i))
            .map(|t| &t.note)
    }

    pub fn get_editor(&self) -> &TextArea<'static> {
        &self
            .tab_index
            .and_then(|i| self.tabs.get(i))
            .log_expect("no opened note")
            .editor
    }

    pub fn get_editor_mut(&mut self) -> &mut TextArea<'static> {
        &mut self
            .tab_index
            .and_then(|i| self.tabs.get_mut(i))
            .log_expect("no opened note")
            .editor
    }

    pub fn close_tab(&mut self, note_id: &NoteId) {
        self.tabs.retain(|tab| &tab.note.id != note_id);
    }

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

    pub fn select_next(&mut self, step: usize) {
        let i = match self.tree_state.selected().unwrap_or_default() + step {
            i if i >= self.tree_items.len() => self.tree_items.len() - 1,
            i => i,
        };

        self.tree_state.select(Some(i));
    }

    pub fn select_prev(&mut self, step: usize) {
        let i = self
            .tree_state
            .selected()
            .unwrap_or_default()
            .saturating_sub(step);

        self.tree_state.select(Some(i));
    }

    pub fn selected(&self) -> &TreeItem {
        self.tree_state
            .selected()
            .and_then(|i| self.tree_items.get(i))
            .log_expect("[NotebookContext::selected] selected must not be empty")
    }

    pub fn selected_name(&self) -> String {
        match self.selected() {
            TreeItem::Directory { value, .. } => value.name.clone(),
            TreeItem::Note { value, .. } => value.name.clone(),
        }
    }

    pub fn open_note(&mut self, note: Note, content: String) {
        let i = self.tabs.iter().enumerate().find_map(|(i, tab)| {
            if tab.note.id == note.id {
                Some(i)
            } else {
                None
            }
        });

        if let Some(i) = i {
            self.tab_index = Some(i);
        } else {
            let tab = EditorTab {
                note,
                editor: TextArea::from(content.lines()),
            };
            self.tabs.push(tab);
            self.tab_index = Some(self.tabs.len() - 1);
        }

        let yank = self.yank.clone();
        if let Some(yank) = yank {
            self.get_editor_mut().set_yank_text(yank);
        }
    }

    pub fn consume(&mut self, input: &Input) -> Action {
        let code = match input {
            Input::Key(key) => key.code,
            _ => return Action::None,
        };

        match self.state {
            ContextState::NoteTreeBrowsing => self.consume_on_note_tree_browsing(code),
            ContextState::NoteTreeNumbering => Action::PassThrough,
            ContextState::EditorNormalMode { idle } => self.consume_on_editor_normal(input, idle),
            ContextState::EditorVisualMode => Action::PassThrough,
            ContextState::EditorInsertMode => self.consume_on_editor_insert(input),
            ContextState::NoteActionsDialog => self.consume_on_note_actions(code),
            ContextState::DirectoryActionsDialog => self.consume_on_directory_actions(code),
        }
    }

    fn consume_on_note_tree_browsing(&mut self, code: KeyCode) -> Action {
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
                TreeItem::Note { .. } => Action::PassThrough,
            },
            KeyCode::Char('m') => match item!() {
                TreeItem::Directory { .. } => {
                    self.directory_actions_state.select_first();

                    Action::PassThrough
                }
                TreeItem::Note { .. } => {
                    self.note_actions_state.select_first();

                    Action::PassThrough
                }
            },
            KeyCode::Esc => TuiAction::Confirm {
                message: "Do you want to quit?".to_owned(),
                action: Box::new(TuiAction::Quit.into()),
            }
            .into(),
            _ => Action::PassThrough,
        }
    }

    fn consume_on_editor_normal(&mut self, input: &Input, idle: bool) -> Action {
        let code = match input {
            Input::Key(key) => key.code,
            _ => return Action::None,
        };

        match code {
            KeyCode::Esc if idle => TuiAction::SaveAndConfirm {
                message: "Do you want to quit?".to_owned(),
                action: Box::new(TuiAction::Quit.into()),
            }
            .into(),
            KeyCode::Char('n') if idle => {
                self.show_browser = true;
                self.yank = Some(self.get_editor_mut().yank_text());

                TuiAction::SaveAndPassThrough.into()
            }
            _ => Action::PassThrough,
        }
    }

    fn consume_on_editor_insert(&mut self, input: &Input) -> Action {
        match input {
            Input::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => Action::Dispatch(NotebookEvent::ViewNote.into()),
            Input::Key(KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => TuiAction::ShowEditorKeymap.into(),
            Input::Key(KeyEvent {
                code: KeyCode::Char('c' | 'x' | 'w' | 'k' | 'j'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                self.line_yanked = false;
                self.get_editor_mut().input(input.clone());
                Action::None
            }
            _ => {
                self.get_editor_mut().input(input.clone());
                Action::None
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
            KeyCode::Esc => Action::Dispatch(NotebookEvent::CloseNoteActionsDialog.into()),
            KeyCode::Enter => {
                match NOTE_ACTIONS[self
                    .note_actions_state
                    .selected()
                    .log_expect("note action must not be empty")]
                {
                    RENAME_NOTE => TuiAction::Prompt {
                        message: vec![Line::raw("Enter new note name:")],
                        action: Box::new(TuiAction::RenameNote.into()),
                        default: Some(self.selected_name()),
                    }
                    .into(),
                    REMOVE_NOTE => TuiAction::Confirm {
                        message: "Confirm to remove note?".to_owned(),
                        action: Box::new(TuiAction::RemoveNote.into()),
                    }
                    .into(),
                    CLOSE => Action::Dispatch(NotebookEvent::CloseNoteActionsDialog.into()),
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
                        default: Some(self.selected_name()),
                    }
                    .into(),
                    REMOVE_DIRECTORY => TuiAction::Confirm {
                        message: "Confirm to remove directory?".to_owned(),
                        action: Box::new(TuiAction::RemoveDirectory.into()),
                    }
                    .into(),
                    CLOSE => Action::Dispatch(NotebookEvent::CloseDirectoryActionsDialog.into()),
                    _ => Action::None,
                }
            }
            KeyCode::Esc => Action::Dispatch(NotebookEvent::CloseDirectoryActionsDialog.into()),
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
