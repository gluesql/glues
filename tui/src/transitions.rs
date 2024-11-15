use {
    super::{
        context::{self, notebook::TreeItem, ContextState},
        logger::*,
        App,
    },
    glues_core::{
        data::{Directory, Note},
        state::{
            notebook::{InnerState, VimNormalState},
            GetInner, NotebookState,
        },
        transition::{
            EntryTransition, NormalModeTransition, NotebookTransition, Transition,
            VisualModeTransition,
        },
        NotebookEvent,
    },
    std::time::SystemTime,
    tui_textarea::{CursorMove, TextArea},
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
        use context::notebook::ContextState;

        let NotebookState {
            root,
            inner_state,
            tab_index,
            ..
        } = self.glues.state.get_inner().log_unwrap();
        let new_state = match inner_state {
            InnerState::NoteSelected | InnerState::DirectorySelected => {
                ContextState::NoteTreeBrowsing
            }
            InnerState::NoteTreeNumber(_) => ContextState::NoteTreeNumbering,
            InnerState::NoteMoreActions => ContextState::NoteActionsDialog,
            InnerState::DirectoryMoreActions => ContextState::DirectoryActionsDialog,
            InnerState::EditingNormalMode(VimNormalState::Idle) => {
                ContextState::EditorNormalMode { idle: true }
            }
            InnerState::EditingNormalMode(_) => ContextState::EditorNormalMode { idle: false },
            InnerState::EditingVisualMode(_) => ContextState::EditorVisualMode,
            InnerState::EditingInsertMode => ContextState::EditorInsertMode,
        };

        if self.context.notebook.state != new_state {
            self.context.notebook.state = new_state;
        }

        if &self.context.notebook.tab_index != tab_index {
            self.context.notebook.tab_index = *tab_index;
        }

        match transition {
            NotebookTransition::ShowVimKeymap(kind) => {
                self.context.vim_keymap = Some(kind);
            }
            NotebookTransition::OpenDirectory { id, .. } => {
                log!("Opening directory {id}");
                self.context.notebook.update_items(root);
            }
            NotebookTransition::CloseDirectory(id) => {
                log!("Closing directory {id}");
                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&id);
            }
            NotebookTransition::OpenNote { note, content } => {
                self.context.notebook.open_note(note, content);
            }
            NotebookTransition::ViewMode(_note) => {
                self.save().await;
            }
            NotebookTransition::BrowseNoteTree => {}
            NotebookTransition::RemoveNote {
                selected_directory, ..
            }
            | NotebookTransition::RemoveDirectory {
                selected_directory, ..
            } => {
                self.context.notebook.select_item(&selected_directory.id);
                self.context.notebook.update_items(root);
            }
            NotebookTransition::RenameNote(_) | NotebookTransition::RenameDirectory(_) => {
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
            NotebookTransition::EditingNormalMode(transition) => {
                self.handle_normal_mode_transition(transition);
            }
            NotebookTransition::EditingVisualMode(transition) => {
                self.handle_visual_mode_transition(transition);
            }
            NotebookTransition::Alert(message) => {
                log!("[Alert] {message}");
                self.context.alert = Some(message);
            }
            _ => {}
        }
    }

    fn handle_normal_mode_transition(&mut self, transition: NormalModeTransition) {
        use NormalModeTransition::*;

        match transition {
            IdleMode => {
                self.context.notebook.get_editor_mut().cancel_selection();
            }
            ToggleMode | NumberingMode | GatewayMode | YankMode | DeleteMode | DeleteInsideMode
            | ChangeMode | ChangeInsideMode => {}
            NextTab(note_id) | PrevTab(note_id) => {
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&note_id);
            }
            CloseTab(note_id) => {
                self.context.notebook.close_tab(&note_id);

                let state: &NotebookState = self.glues.state.get_inner().log_unwrap();
                self.context.notebook.update_items(&state.root);

                let note_id = &state.get_selected_note().log_unwrap().id;
                self.context.notebook.update_items(&state.root);
                self.context.notebook.select_item(note_id);
            }
            ToggleLineNumbers => {
                self.context.notebook.show_line_number = !self.context.notebook.show_line_number;
            }
            ToggleBrowser => {
                self.context.notebook.show_browser = !self.context.notebook.show_browser;
            }
            MoveCursorDown(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let cursor_move = cursor_move_down(editor, n);

                editor.move_cursor(cursor_move);
            }
            MoveCursorUp(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let cursor_move = cursor_move_up(editor, n);

                editor.move_cursor(cursor_move);
            }
            MoveCursorBack(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let (row, col) = editor.cursor();
                let cursor_move = if col < n {
                    CursorMove::Head
                } else {
                    CursorMove::Jump(row as u16, (col - n) as u16)
                };

                editor.move_cursor(cursor_move);
            }
            MoveCursorForward(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let cursor_move = cursor_move_forward(editor, n);

                editor.move_cursor(cursor_move);
            }
            MoveCursorWordForward(n) => {
                let editor = self.context.notebook.get_editor_mut();

                for _ in 0..n {
                    editor.move_cursor(CursorMove::WordForward);
                }
            }
            MoveCursorWordEnd(n) => {
                move_cursor_word_end(self.context.notebook.get_editor_mut(), n);
            }
            MoveCursorWordBack(n) => {
                move_cursor_word_back(self.context.notebook.get_editor_mut(), n);
            }
            MoveCursorLineStart => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::Head);
            }
            MoveCursorLineEnd => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::End);
            }
            MoveCursorLineNonEmptyStart => {
                move_cursor_to_line_non_empty_start(self.context.notebook.get_editor_mut());
            }
            MoveCursorTop => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::Top);
            }
            MoveCursorBottom => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::Bottom);
            }
            MoveCursorToLine(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.move_cursor(CursorMove::Jump((n - 1) as u16, 0));
                editor.move_cursor(CursorMove::WordForward);
            }
            InsertNewLineBelow => {
                let editor = self.context.notebook.get_editor_mut();
                editor.move_cursor(CursorMove::End);
                editor.insert_newline();
            }
            InsertNewLineAbove => {
                let editor = self.context.notebook.get_editor_mut();
                editor.move_cursor(CursorMove::Head);
                editor.insert_newline();
                editor.move_cursor(CursorMove::Up);
            }
            InsertAtCursor => {}
            InsertAtLineStart => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::Head);
            }
            InsertAfterCursor => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::Forward);
            }
            InsertAtLineEnd => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::End);
            }
            DeleteChars(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                let cursor_move = cursor_move_forward(editor, n);

                editor.move_cursor(cursor_move);
                editor.cut();
            }
            Paste => {
                let line_yanked = self.context.notebook.line_yanked;
                let editor = self.context.notebook.get_editor_mut();
                if line_yanked {
                    editor.move_cursor(CursorMove::End);
                    editor.insert_newline();
                    editor.paste();
                    move_cursor_to_line_non_empty_start(editor);
                } else {
                    editor.paste();
                }
            }
            Undo => {
                self.context.notebook.get_editor_mut().undo();
            }
            Redo => {
                self.context.notebook.get_editor_mut().redo();
            }
            YankLines(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let cursor = editor.cursor();
                editor.move_cursor(CursorMove::Head);
                editor.start_selection();
                let cursor_move = cursor_move_down(editor, n - 1);
                editor.move_cursor(cursor_move);
                editor.move_cursor(CursorMove::End);
                editor.copy();
                editor.cancel_selection();
                editor.move_cursor(CursorMove::Jump(cursor.0 as u16, cursor.1 as u16));
                self.context.notebook.line_yanked = true;
            }
            DeleteLines(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let (row, _) = editor.cursor();

                editor.move_cursor(CursorMove::Head);
                editor.start_selection();
                let cursor_move = cursor_move_down(editor, n - 1);
                editor.move_cursor(cursor_move);
                editor.move_cursor(CursorMove::End);
                editor.cut();

                if row == 0 {
                    editor.move_cursor(CursorMove::Down);
                    editor.move_cursor(CursorMove::Head);
                    editor.delete_char();
                } else {
                    editor.delete_char();
                    editor.move_cursor(CursorMove::Down);
                }

                move_cursor_to_line_non_empty_start(editor);
                self.context.notebook.line_yanked = true;
            }
            DeleteLinesAndInsert(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.move_cursor(CursorMove::Head);
                editor.start_selection();
                let cursor_move = cursor_move_down(editor, n - 1);
                editor.move_cursor(cursor_move);
                editor.move_cursor(CursorMove::End);
                editor.cut();
                self.context.notebook.line_yanked = true;
            }
            DeleteInsideWord(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let cursor = editor.cursor();
                editor.move_cursor(CursorMove::WordBack);
                editor.move_cursor(CursorMove::WordEnd);
                let cursor_be = editor.cursor();
                editor.move_cursor(CursorMove::Jump(cursor.0 as u16, cursor.1 as u16));

                if cursor_be >= cursor {
                    editor.move_cursor(CursorMove::WordBack);
                }

                editor.start_selection();
                move_cursor_word_end(editor, n);
                editor.move_cursor(CursorMove::Forward);
                editor.cut();

                self.context.notebook.line_yanked = false;
            }
            DeleteWordEnd(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                move_cursor_word_end(editor, n);
                editor.move_cursor(CursorMove::Forward);
                editor.cut();

                self.context.notebook.line_yanked = false;
            }
            DeleteWordBack(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                move_cursor_word_back(editor, n);
                editor.cut();

                self.context.notebook.line_yanked = false;
            }
            DeleteLineStart => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                editor.move_cursor(CursorMove::Head);
                editor.cut();

                self.context.notebook.line_yanked = false;
            }
            DeleteLineEnd(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                let cursor_move = cursor_move_down(editor, n - 1);
                editor.move_cursor(cursor_move);
                editor.move_cursor(CursorMove::End);
                editor.cut();

                self.context.notebook.line_yanked = false;
            }
        };
    }

    fn handle_visual_mode_transition(&mut self, transition: VisualModeTransition) {
        use VisualModeTransition::*;

        match transition {
            IdleMode => {
                self.context.notebook.get_editor_mut().start_selection();
            }
            NumberingMode | GatewayMode => {}
            MoveCursorDown(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let cursor_move = cursor_move_down(editor, n);

                editor.move_cursor(cursor_move);
            }
            MoveCursorUp(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let cursor_move = cursor_move_up(editor, n);

                editor.move_cursor(cursor_move);
            }
            MoveCursorBack(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let (row, col) = editor.cursor();
                let cursor_move = if col < n {
                    CursorMove::Head
                } else {
                    CursorMove::Jump(row as u16, (col - n) as u16)
                };

                editor.move_cursor(cursor_move);
            }
            MoveCursorForward(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let cursor_move = cursor_move_forward(editor, n);

                editor.move_cursor(cursor_move);
            }
            MoveCursorWordForward(n) => {
                let editor = self.context.notebook.get_editor_mut();

                for _ in 0..n {
                    editor.move_cursor(CursorMove::WordForward);
                }
            }
            MoveCursorWordEnd(n) => {
                let editor = self.context.notebook.get_editor_mut();

                for _ in 0..n {
                    editor.move_cursor(CursorMove::WordEnd);
                }
            }
            MoveCursorWordBack(n) => {
                let editor = self.context.notebook.get_editor_mut();

                for _ in 0..n {
                    editor.move_cursor(CursorMove::WordBack);
                }
            }
            MoveCursorLineStart => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::Head);
            }
            MoveCursorLineEnd => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::End);
            }
            MoveCursorLineNonEmptyStart => {
                move_cursor_to_line_non_empty_start(self.context.notebook.get_editor_mut());
            }
            MoveCursorBottom => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::Bottom);
            }
            MoveCursorTop => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .move_cursor(CursorMove::Top);
            }
            MoveCursorToLine(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.move_cursor(CursorMove::Jump((n - 1) as u16, 0));
                editor.move_cursor(CursorMove::WordForward);
            }
            YankSelection => {
                let editor = self.context.notebook.get_editor_mut();
                reselect_for_yank(editor);
                editor.copy();
                self.context.notebook.line_yanked = false;
            }
            DeleteSelection => {
                let editor = self.context.notebook.get_editor_mut();
                reselect_for_yank(editor);
                editor.cut();
                self.context.notebook.line_yanked = false;
            }
            DeleteSelectionAndInsertMode => {
                let editor = self.context.notebook.get_editor_mut();
                reselect_for_yank(editor);
                editor.cut();
                self.context.notebook.line_yanked = false;
            }
        }
    }

    pub(crate) async fn save(&mut self) {
        let content = self.context.notebook.get_editor().lines().join("\n");
        let event = NotebookEvent::UpdateNoteContent(content).into();

        self.glues.dispatch(event).await.log_unwrap();
    }
}

fn cursor_move_forward(editor: &TextArea, n: usize) -> CursorMove {
    let (row, col) = editor.cursor();
    if col + n >= editor.lines()[row].len() {
        CursorMove::End
    } else {
        CursorMove::Jump(row as u16, (col + n) as u16)
    }
}

fn cursor_move_down(editor: &TextArea, n: usize) -> CursorMove {
    let num_lines = editor.lines().len();
    let (row, col) = editor.cursor();
    if row + n >= num_lines {
        CursorMove::Bottom
    } else {
        CursorMove::Jump((row + n) as u16, col as u16)
    }
}

fn cursor_move_up(editor: &TextArea, n: usize) -> CursorMove {
    let (row, col) = editor.cursor();
    if row < n {
        CursorMove::Top
    } else {
        CursorMove::Jump((row - n) as u16, col as u16)
    }
}

fn move_cursor_to_line_non_empty_start(editor: &mut TextArea) {
    editor.move_cursor(CursorMove::Head);

    let (row, _) = editor.cursor();
    let is_whitespace_at_first = editor.lines()[row]
        .chars()
        .next()
        .map(|c| c.is_whitespace())
        .unwrap_or(false);
    if is_whitespace_at_first {
        editor.move_cursor(CursorMove::WordForward);
    }
}

fn move_cursor_word_end(editor: &mut TextArea, n: usize) {
    for _ in 0..n {
        editor.move_cursor(CursorMove::WordEnd);
    }
}

fn move_cursor_word_back(editor: &mut TextArea, n: usize) {
    for _ in 0..n {
        editor.move_cursor(CursorMove::WordBack);
    }
}

fn reselect_for_yank(editor: &mut TextArea) {
    let (begin, end) = match editor.selection_range() {
        None => return,
        Some(range) => range,
    };

    editor.cancel_selection();
    editor.move_cursor(CursorMove::Jump(begin.0 as u16, begin.1 as u16));
    editor.start_selection();
    editor.move_cursor(CursorMove::Jump(end.0 as u16, end.1 as u16));
    editor.move_cursor(CursorMove::Forward);
}
