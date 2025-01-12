mod tree_item;

use {
    crate::{
        action::{Action, TuiAction},
        logger::*,
    },
    glues_core::{
        data::Note,
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

pub use tree_item::{TreeItem, TreeItemKind};

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
    MoveMode,
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
    pub editor_height: u16,
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
    pub dirty: bool,
    pub breadcrumb: Vec<String>,
}

impl Default for NotebookContext {
    fn default() -> Self {
        Self {
            state: ContextState::NoteTreeBrowsing,
            tree_state: ListState::default().with_selected(Some(0)),
            tree_items: vec![],

            note_actions_state: ListState::default(),
            directory_actions_state: ListState::default(),

            editor_height: 0,
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
            .log_expect("[NotebookContext::get_editor] no opened note")
            .editor
    }

    pub fn get_editor_mut(&mut self) -> &mut TextArea<'static> {
        &mut self
            .tab_index
            .and_then(|i| self.tabs.get_mut(i))
            .log_expect("[NotebookContext::get_editor_mut] no opened note")
            .editor
    }

    pub fn mark_dirty(&mut self) {
        if let Some(tab) = self.tab_index.and_then(|i| self.tabs.get_mut(i)) {
            tab.dirty = true;
        }
    }

    pub fn mark_clean(&mut self, note_id: &NoteId) {
        for tab in self.tabs.iter_mut() {
            if &tab.note.id == note_id {
                tab.dirty = false;
                break;
            }
        }
    }

    pub fn close_tab(&mut self, note_id: &NoteId) {
        self.tabs.retain(|tab| &tab.note.id != note_id);
    }

    pub fn update_items(&mut self, directory_item: &DirectoryItem) {
        self.tree_items = self.flatten(directory_item, 0, true);
    }

    fn flatten(
        &self,
        directory_item: &DirectoryItem,
        depth: usize,
        selectable: bool,
    ) -> Vec<TreeItem> {
        let id = self
            .tree_state
            .selected()
            .and_then(|i| self.tree_items.get(i))
            .map(|item| item.id());
        let is_move_mode = matches!(self.state, ContextState::MoveMode { .. });
        let selectable = !is_move_mode || (selectable && Some(&directory_item.directory.id) != id);

        let mut items = vec![TreeItem {
            depth,
            target: Some(&directory_item.directory.id) == id,
            selectable,
            kind: TreeItemKind::Directory {
                directory: directory_item.directory.clone(),
                opened: directory_item.children.is_some(),
            },
        }];

        if let Some(children) = &directory_item.children {
            for item in &children.directories {
                items.extend(self.flatten(item, depth + 1, selectable));
            }

            for note in &children.notes {
                items.push(TreeItem {
                    depth: depth + 1,
                    target: Some(&note.id) == id,
                    selectable: !is_move_mode,
                    kind: TreeItemKind::Note { note: note.clone() },
                })
            }
        }

        items
    }

    pub fn select_item(&mut self, id: &Id) {
        for (i, item) in self.tree_items.iter().enumerate() {
            if item.id() == id {
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

        let i = self
            .tree_items
            .iter()
            .enumerate()
            .skip(i)
            .find(|(_, item)| item.selectable)
            .map(|(i, _)| i);

        if i.is_some() {
            self.tree_state.select(i);
        }
    }

    pub fn select_prev(&mut self, step: usize) {
        let i = self
            .tree_state
            .selected()
            .unwrap_or_default()
            .saturating_sub(step);

        let i = self
            .tree_items
            .iter()
            .enumerate()
            .rev()
            .skip(self.tree_items.len() - i - 1)
            .find(|(_, item)| item.selectable)
            .map(|(i, _)| i);

        if i.is_some() {
            self.tree_state.select(i);
        }
    }

    pub fn selected(&self) -> &TreeItem {
        self.tree_state
            .selected()
            .and_then(|i| self.tree_items.get(i))
            .log_expect("[NotebookContext::selected] selected must not be empty")
    }

    pub fn breadcrumb(&self, note: &Note) -> Vec<String> {
        let mut breadcrumb = vec![note.name.clone()];
        let (i, mut depth) = self
            .tree_items
            .iter()
            .enumerate()
            .find_map(|(i, item)| (item.id() == &note.id).then_some((i, item.depth)))
            .log_expect("[NotebookContext::open_note] note not found");

        self.tree_items[0..i].iter().rev().for_each(|item| {
            if item.depth < depth {
                depth = item.depth;

                breadcrumb.push(item.name().clone());
            }
        });
        breadcrumb.reverse();
        breadcrumb
    }

    pub fn refresh_breadcrumbs(&mut self) {
        let mut breadcrumbs = self
            .tabs
            .iter()
            .map(|tab| self.breadcrumb(&tab.note))
            .collect::<Vec<_>>();

        breadcrumbs
            .iter_mut()
            .enumerate()
            .for_each(|(i, breadcrumb)| {
                if let Some(tab) = self.tabs.get_mut(i) {
                    tab.breadcrumb = breadcrumb.clone();
                }
            });
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
            let breadcrumb = self.breadcrumb(&note);
            let tab = EditorTab {
                note,
                editor: TextArea::from(content.lines()),
                dirty: false,
                breadcrumb,
            };
            self.tabs.push(tab);
            self.tab_index = Some(self.tabs.len() - 1);
        }

        self.apply_yank();
    }

    pub fn apply_yank(&mut self) {
        if self.tabs.is_empty() {
            return;
        }

        if let Some(yank) = self.yank.clone() {
            self.get_editor_mut().set_yank_text(yank);
        }
    }

    pub fn update_yank(&mut self) {
        self.yank = Some(self.get_editor().yank_text());
    }

    pub fn consume(&mut self, input: &Input) -> Action {
        let code = match input {
            Input::Key(key) => key.code,
            _ => return Action::None,
        };

        match self.state {
            ContextState::NoteTreeBrowsing => self.consume_on_note_tree_browsing(code),
            ContextState::NoteTreeNumbering | ContextState::MoveMode => Action::PassThrough,
            ContextState::EditorNormalMode { idle } => self.consume_on_editor_normal(input, idle),
            ContextState::EditorVisualMode => Action::PassThrough,
            ContextState::EditorInsertMode => self.consume_on_editor_insert(input),
            ContextState::NoteActionsDialog => self.consume_on_note_actions(code),
            ContextState::DirectoryActionsDialog => self.consume_on_directory_actions(code),
        }
    }

    fn consume_on_note_tree_browsing(&mut self, code: KeyCode) -> Action {
        match code {
            KeyCode::Char('m') => {
                if self
                    .tree_state
                    .selected()
                    .and_then(|idx| self.tree_items.get(idx))
                    .log_expect("[NotebookContext::consume] selected must not be empty")
                    .is_directory()
                {
                    self.directory_actions_state.select_first();
                } else {
                    self.note_actions_state.select_first();
                }

                Action::PassThrough
            }
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
            KeyCode::Tab if idle => {
                self.show_browser = true;
                self.update_yank();

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
            KeyCode::Char('j') | KeyCode::Down => {
                self.note_actions_state.select_next();
                Action::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
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
                        default: Some(self.selected().name()),
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
            KeyCode::Char('j') | KeyCode::Down => {
                self.directory_actions_state.select_next();
                Action::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
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
                        default: Some(self.selected().name()),
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
