use {
    crate::{action::Action, logger::*},
    edtui::{EditorEventHandler, EditorMode, EditorState, Lines},
    glues_core::{
        data::{Directory, Note},
        state::notebook::DirectoryItem,
        NotebookEvent,
    },
    ratatui::{
        crossterm::event::{Event, KeyCode},
        widgets::ListState,
    },
};

pub struct NotebookContext {
    pub tree_state: ListState,
    pub tree_items: Vec<TreeItem>,

    pub editor_state: EditorState,
    pub editor_handler: EditorEventHandler,
    pub opened_note: Option<Note>,
}

impl NotebookContext {
    pub fn new() -> Self {
        Self {
            tree_state: ListState::default().with_selected(Some(0)),
            tree_items: vec![],
            editor_state: EditorState::new(Lines::from("\n   >\")++++<")),
            editor_handler: EditorEventHandler::default(),
            opened_note: None,
        }
    }

    pub fn update_items(&mut self, directory_item: &DirectoryItem) {
        self.tree_items = flatten(directory_item, 0);
    }

    pub fn open_note(&mut self, note: Note, content: String) {
        self.opened_note = Some(note);
        self.editor_state = EditorState::new(Lines::from(content.as_str()));
    }

    pub fn consume(&mut self, code: KeyCode) -> Action {
        if self.opened_note.is_some() {
            self.consume_on_editor(code)
        } else {
            self.consume_on_note_tree(code)
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
            KeyCode::Char('h') => Action::None,
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
            KeyCode::Char('o') | KeyCode::Char('b') | KeyCode::Char('e') | KeyCode::Esc => {
                Action::PassThrough
            }
            _ => Action::None,
        }
    }

    fn consume_on_editor(&mut self, code: KeyCode) -> Action {
        let mode = self.editor_state.mode;

        match code {
            KeyCode::Char('/') | KeyCode::Char('v') if mode == EditorMode::Normal => Action::None,
            KeyCode::Char('b') if mode == EditorMode::Normal => {
                self.opened_note = None;

                Action::Dispatch(NotebookEvent::BrowseNoteTree.into())
            }
            KeyCode::Esc if mode == EditorMode::Insert => {
                self.editor_handler
                    .on_event(Event::Key(code.into()), &mut self.editor_state);

                Action::Dispatch(NotebookEvent::ViewNote.into())
            }
            _ => {
                self.editor_handler
                    .on_event(Event::Key(code.into()), &mut self.editor_state);

                let new_mode = self.editor_state.mode;
                if mode != new_mode {
                    match new_mode {
                        EditorMode::Normal => Action::Dispatch(NotebookEvent::ViewNote.into()),
                        EditorMode::Insert => Action::Dispatch(NotebookEvent::EditNote.into()),
                        _ => Action::None,
                    }
                } else {
                    Action::None
                }
            }
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
