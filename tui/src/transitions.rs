use {
    super::{
        context::{self, notebook::TreeItem, ContextState},
        logger::*,
        App,
    },
    glues_core::{
        data::{Directory, Note},
        state::{GetInner, NotebookState},
        transition::{EntryTransition, NormalModeTransition, NotebookTransition, Transition},
        NotebookEvent,
    },
    std::time::SystemTime,
    tui_textarea::CursorMove,
};

impl App {
    pub(super) async fn handle_transition(&mut self, transition: Transition) {
        match transition {
            Transition::Entry(transition) => {
                self.handle_entry_transition(transition).await;
            }
            Transition::Notebook(transition) => {
                self.handle_notebook_transition(transition).await;
            }
            Transition::Log(message) => {
                log!("{message}");
                self.context.last_log = Some((message, SystemTime::now()));
            }
            Transition::Error(message) => {
                log!("[Err] {message}");
                self.context.alert = Some(message);
            }
        }
    }

    pub(super) async fn handle_entry_transition(&mut self, transition: EntryTransition) {
        match transition {
            EntryTransition::OpenNotebook => {
                log!("Opening notebook");

                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();
                self.context.state = ContextState::Notebook;
                self.context.notebook.update_items(root);
            }
            EntryTransition::Inedible(event) => {
                log!("Inedible event: {event}");
            }
            EntryTransition::None => {}
        }
    }

    pub(super) async fn handle_notebook_transition(&mut self, transition: NotebookTransition) {
        match transition {
            NotebookTransition::OpenDirectory { id, .. } => {
                log!("Opening directory {id}");
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();
                self.context.notebook.update_items(root);
            }
            NotebookTransition::CloseDirectory(id) => {
                log!("Closing directory {id}");
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();
                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&id);
            }
            NotebookTransition::OpenNote { note, content } => {
                self.context.notebook.open_note(note, content);
            }
            NotebookTransition::ViewMode(_note) => {
                let content = self.context.notebook.editor.lines().join("\n");
                let event = NotebookEvent::UpdateNoteContent(content).into();

                self.glues.dispatch(event).await.log_unwrap();
            }
            NotebookTransition::RemoveNote {
                selected_directory, ..
            }
            | NotebookTransition::RemoveDirectory {
                selected_directory, ..
            } => {
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.select_item(&selected_directory.id);
                self.context.notebook.state = context::notebook::ContextState::NoteTreeBrowsing;
                self.context.notebook.update_items(root);
            }
            NotebookTransition::RenameNote(_) | NotebookTransition::RenameDirectory(_) => {
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.state = context::notebook::ContextState::NoteTreeBrowsing;
                self.context.notebook.update_items(root);
            }
            NotebookTransition::AddNote(Note {
                id,
                directory_id: parent_id,
                ..
            })
            | NotebookTransition::AddDirectory(Directory { id, parent_id, .. }) => {
                self.glues
                    .dispatch(NotebookEvent::OpenDirectory(parent_id.clone()).into())
                    .await
                    .log_unwrap();
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.state = context::notebook::ContextState::NoteTreeBrowsing;
                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&id);
            }
            NotebookTransition::SelectNext(n) => {
                self.context.notebook.select_next(n);

                let event = match self.context.notebook.selected() {
                    TreeItem::Directory { value, .. } => {
                        NotebookEvent::SelectDirectory(value.clone()).into()
                    }
                    TreeItem::Note { value, .. } => NotebookEvent::SelectNote(value.clone()).into(),
                };

                self.glues.dispatch(event).await.log_unwrap();
            }
            NotebookTransition::SelectPrev(n) => {
                self.context.notebook.select_prev(n);

                let event = match self.context.notebook.selected() {
                    TreeItem::Directory { value, .. } => {
                        NotebookEvent::SelectDirectory(value.clone()).into()
                    }
                    TreeItem::Note { value, .. } => NotebookEvent::SelectNote(value.clone()).into(),
                };

                self.glues.dispatch(event).await.log_unwrap();
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::IdleMode) => {
                self.context.notebook.state =
                    context::notebook::ContextState::EditorNormalMode { idle: true };
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::NumberingMode) => {
                self.context.notebook.state =
                    context::notebook::ContextState::EditorNormalMode { idle: false };
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorDown(n)) => {
                let num_lines = self.context.notebook.editor.lines().len();
                let (row, col) = self.context.notebook.editor.cursor();
                let cursor_move = if row + n >= num_lines {
                    CursorMove::Bottom
                } else {
                    CursorMove::Jump((row + n) as u16, col as u16)
                };

                self.context.notebook.editor.move_cursor(cursor_move);
                self.context.notebook.state =
                    context::notebook::ContextState::EditorNormalMode { idle: true };
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorUp(n)) => {
                let (row, col) = self.context.notebook.editor.cursor();
                let cursor_move = if row < n {
                    CursorMove::Top
                } else {
                    CursorMove::Jump((row - n) as u16, col as u16)
                };

                self.context.notebook.editor.move_cursor(cursor_move);
                self.context.notebook.state =
                    context::notebook::ContextState::EditorNormalMode { idle: true };
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorBack(n)) => {
                let (row, col) = self.context.notebook.editor.cursor();
                let cursor_move = if col < n {
                    CursorMove::Head
                } else {
                    CursorMove::Jump(row as u16, (col - n) as u16)
                };

                self.context.notebook.editor.move_cursor(cursor_move);
                self.context.notebook.state =
                    context::notebook::ContextState::EditorNormalMode { idle: true };
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorForward(n)) => {
                let editor = &mut self.context.notebook.editor;
                let (row, col) = editor.cursor();
                let cursor_move = if col + n >= editor.lines()[row].len() {
                    CursorMove::End
                } else {
                    CursorMove::Jump(row as u16, (col + n) as u16)
                };

                editor.move_cursor(cursor_move);
                self.context.notebook.state =
                    context::notebook::ContextState::EditorNormalMode { idle: true };
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorWordForward(
                n,
            )) => {
                for _ in 0..n {
                    self.context
                        .notebook
                        .editor
                        .move_cursor(CursorMove::WordForward);
                }

                self.context.notebook.state =
                    context::notebook::ContextState::EditorNormalMode { idle: true };
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorWordEnd(n)) => {
                for _ in 0..n {
                    self.context
                        .notebook
                        .editor
                        .move_cursor(CursorMove::WordEnd);
                }

                self.context.notebook.state =
                    context::notebook::ContextState::EditorNormalMode { idle: true };
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorWordBack(n)) => {
                for _ in 0..n {
                    self.context
                        .notebook
                        .editor
                        .move_cursor(CursorMove::WordBack);
                }

                self.context.notebook.state =
                    context::notebook::ContextState::EditorNormalMode { idle: true };
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorLineStart) => {
                self.context.notebook.editor.move_cursor(CursorMove::Head);
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorLineEnd) => {
                self.context.notebook.editor.move_cursor(CursorMove::End);
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorBottom) => {
                self.context.notebook.editor.move_cursor(CursorMove::Bottom);
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::MoveCursorToLine(n)) => {
                self.context
                    .notebook
                    .editor
                    .move_cursor(CursorMove::Jump((n - 1) as u16, 0));
                self.context
                    .notebook
                    .editor
                    .move_cursor(CursorMove::WordForward);
                self.context.notebook.state =
                    context::notebook::ContextState::EditorNormalMode { idle: true };
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::InsertNewLineBelow) => {
                self.context.notebook.editor.move_cursor(CursorMove::End);
                self.context.notebook.editor.insert_newline();
                self.context.notebook.state = context::notebook::ContextState::EditorInsertMode;
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::InsertNewLineAbove) => {
                self.context.notebook.editor.move_cursor(CursorMove::Head);
                self.context.notebook.editor.insert_newline();
                self.context.notebook.editor.move_cursor(CursorMove::Up);
                self.context.notebook.state = context::notebook::ContextState::EditorInsertMode;
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::InsertAtCursor) => {
                self.context.notebook.state = context::notebook::ContextState::EditorInsertMode;
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::InsertAtLineStart) => {
                self.context.notebook.editor.move_cursor(CursorMove::Head);
                self.context.notebook.state = context::notebook::ContextState::EditorInsertMode;
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::InsertAfterCursor) => {
                self.context
                    .notebook
                    .editor
                    .move_cursor(CursorMove::Forward);
                self.context.notebook.state = context::notebook::ContextState::EditorInsertMode;
            }
            NotebookTransition::EditingNormalMode(NormalModeTransition::InsertAtLineEnd) => {
                self.context.notebook.editor.move_cursor(CursorMove::End);
                self.context.notebook.state = context::notebook::ContextState::EditorInsertMode;
            }
            NotebookTransition::Alert(message) => {
                log!("[Alert] {message}");
                self.context.alert = Some(message);
            }
            _ => {}
        }
    }
}
