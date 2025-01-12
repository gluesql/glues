use {
    super::{
        context::{
            self,
            notebook::{TreeItem, TreeItemKind},
            ContextState,
        },
        logger::*,
        App,
    },
    async_recursion::async_recursion,
    glues_core::{
        data::{Directory, Note},
        state::{
            notebook::{InnerState, VimNormalState},
            GetInner, NotebookState,
        },
        transition::{
            EntryTransition, MoveModeTransition, NormalModeTransition, NotebookTransition,
            Transition, VisualModeTransition,
        },
        NotebookEvent,
    },
    std::time::SystemTime,
    tui_textarea::{CursorMove, Scrolling, TextArea},
};

impl App {
    #[async_recursion(?Send)]
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
            InnerState::MoveMode => ContextState::MoveMode,
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
                self.context.notebook.mark_dirty();
            }
            NotebookTransition::UpdateNoteContent(note_id) => {
                self.context.notebook.mark_clean(&note_id);
            }
            NotebookTransition::BrowseNoteTree => {}
            NotebookTransition::FocusEditor => {
                let note_id = self
                    .context
                    .notebook
                    .get_opened_note()
                    .log_expect("No note opened")
                    .id
                    .clone();

                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&note_id);
            }
            NotebookTransition::RemoveNote {
                selected_directory, ..
            }
            | NotebookTransition::RemoveDirectory {
                selected_directory, ..
            } => {
                self.context.notebook.select_item(&selected_directory.id);
                self.context.notebook.update_items(root);
            }
            NotebookTransition::RenameDirectory(_) => {
                self.context.notebook.update_items(root);
                self.context.notebook.refresh_breadcrumbs();
            }
            NotebookTransition::RenameNote(note) => {
                self.context.notebook.update_items(root);
                self.context.notebook.tabs.iter_mut().for_each(|tab| {
                    if tab.note.id == note.id {
                        tab.note.name = note.name.clone();
                    }
                });
                self.context.notebook.refresh_breadcrumbs();
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
                    TreeItem {
                        kind: TreeItemKind::Directory { directory, .. },
                        ..
                    } => NotebookEvent::SelectDirectory(directory.clone()).into(),
                    TreeItem {
                        kind: TreeItemKind::Note { note },
                        ..
                    } => NotebookEvent::SelectNote(note.clone()).into(),
                };

                self.glues.dispatch(event).await.log_unwrap();
            }
            NotebookTransition::SelectPrev(n) => {
                self.context.notebook.select_prev(n);

                let event = match self.context.notebook.selected() {
                    TreeItem {
                        kind: TreeItemKind::Directory { directory, .. },
                        ..
                    } => NotebookEvent::SelectDirectory(directory.clone()).into(),
                    TreeItem {
                        kind: TreeItemKind::Note { note },
                        ..
                    } => NotebookEvent::SelectNote(note.clone()).into(),
                };

                self.glues.dispatch(event).await.log_unwrap();
            }
            NotebookTransition::EditingNormalMode(transition) => {
                self.handle_normal_mode_transition(transition).await;
            }
            NotebookTransition::EditingVisualMode(transition) => {
                self.handle_visual_mode_transition(transition).await;
            }
            NotebookTransition::MoveMode(transition) => {
                self.handle_move_mode_transition(transition).await;
            }
            NotebookTransition::Alert(message) => {
                log!("[Alert] {message}");
                self.context.alert = Some(message);
            }
            NotebookTransition::ShowNoteActionsDialog(_)
            | NotebookTransition::ShowDirectoryActionsDialog(_)
            | NotebookTransition::Inedible(_)
            | NotebookTransition::None => {}
        }
    }

    async fn handle_move_mode_transition(&mut self, transition: MoveModeTransition) {
        use MoveModeTransition::*;

        match transition {
            Enter => {
                let state: &NotebookState = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.update_items(&state.root);
                self.context.notebook.select_prev(1);
            }
            SelectNext => {
                self.context.notebook.select_next(1);
            }
            SelectPrev => {
                self.context.notebook.select_prev(1);
            }
            RequestCommit => {
                let is_directory = self
                    .context
                    .notebook
                    .tree_items
                    .iter()
                    .find(|item| item.target)
                    .log_expect("No target selected")
                    .is_directory();
                let event = match self.context.notebook.selected() {
                    TreeItem {
                        kind: TreeItemKind::Directory { directory, .. },
                        ..
                    } => {
                        if is_directory {
                            NotebookEvent::MoveDirectory(directory.id.clone()).into()
                        } else {
                            NotebookEvent::MoveNote(directory.id.clone()).into()
                        }
                    }
                    _ => {
                        let message = format!(
                            "Error - Cannot move {} to note",
                            if is_directory { "directory" } else { "note" }
                        );
                        log!("{message}");
                        self.context.alert = Some(message);

                        return;
                    }
                };

                let transition = self.glues.dispatch(event).await.log_unwrap();
                self.handle_transition(transition).await;
            }
            Commit => {
                let state: &NotebookState = self.glues.state.get_inner().log_unwrap();
                let id = state.get_selected_id().log_unwrap();

                self.context.notebook.update_items(&state.root);
                self.context.notebook.select_item(id);
                self.context.notebook.refresh_breadcrumbs();
            }
            Cancel => {
                let state: &NotebookState = self.glues.state.get_inner().log_unwrap();
                let id = state.get_selected_id().log_unwrap();

                self.context.notebook.update_items(&state.root);
                self.context.notebook.select_item(id);
            }
        }
    }

    async fn handle_normal_mode_transition(&mut self, transition: NormalModeTransition) {
        use NormalModeTransition::*;

        match transition {
            IdleMode => {
                self.context.notebook.get_editor_mut().cancel_selection();
            }
            ToggleMode | NumberingMode | GatewayMode | YankMode | DeleteMode | DeleteInsideMode
            | ChangeMode | ChangeInsideMode | ScrollMode => {}
            NextTab(note_id) | PrevTab(note_id) => {
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&note_id);
                self.context.notebook.apply_yank();
            }
            MoveTabNext(i) => {
                let tab = self.context.notebook.tabs.remove(i);
                self.context.notebook.tabs.insert(i + 1, tab);
            }
            MoveTabPrev(i) => {
                let tab = self.context.notebook.tabs.remove(i);
                self.context.notebook.tabs.insert(i - 1, tab);
            }
            CloseTab(note_id) => {
                self.save().await;
                self.context.notebook.close_tab(&note_id);

                let state: &NotebookState = self.glues.state.get_inner().log_unwrap();
                self.context.notebook.update_items(&state.root);

                let note_id = &state.get_selected_note().log_unwrap().id;
                self.context.notebook.update_items(&state.root);
                self.context.notebook.select_item(note_id);
                self.context.notebook.apply_yank();
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
                let cursor_move = cursor_move_back(editor, n);

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
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteCharsBack(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                let cursor_move = cursor_move_back(editor, n);

                editor.move_cursor(cursor_move);
                editor.cut();
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
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

                self.context.notebook.mark_dirty();
            }
            Undo => {
                self.context.notebook.get_editor_mut().undo();
                self.context.notebook.mark_dirty();
            }
            Redo => {
                self.context.notebook.get_editor_mut().redo();
                self.context.notebook.mark_dirty();
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
                self.context.notebook.update_yank();
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
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
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
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
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
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteWordEnd(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                move_cursor_word_end(editor, n);
                editor.move_cursor(CursorMove::Forward);
                editor.cut();

                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteWordBack(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                move_cursor_word_back(editor, n);
                editor.cut();

                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteLineStart => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                editor.move_cursor(CursorMove::Head);
                editor.cut();

                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteLineEnd(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                let cursor_move = cursor_move_down(editor, n - 1);
                editor.move_cursor(cursor_move);
                editor.move_cursor(CursorMove::End);
                editor.cut();

                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            SwitchCase => {
                let editor = self.context.notebook.get_editor_mut();
                editor.start_selection();
                switch_case(editor);

                self.context.notebook.mark_dirty();
            }
            ScrollCenter => {
                let height = self.context.notebook.editor_height;
                let editor = self.context.notebook.get_editor_mut();
                let (row, col) = editor.cursor();
                editor.scroll((i16::MIN / 2, 0));
                editor.scroll(Scrolling::Delta {
                    rows: (row as i16 - height as i16 / 2),
                    cols: 0,
                });
                editor.move_cursor(CursorMove::Jump(row as u16, col as u16));
            }
            ScrollTop => {
                let editor = self.context.notebook.get_editor_mut();
                let (row, col) = editor.cursor();
                editor.move_cursor(CursorMove::Top);
                editor.scroll(Scrolling::Delta {
                    rows: row as i16,
                    cols: 0,
                });
                editor.move_cursor(CursorMove::Head);
                editor.move_cursor(CursorMove::Jump(row as u16, col as u16));
            }
            ScrollBottom => {
                let editor = self.context.notebook.get_editor_mut();
                let (row, col) = editor.cursor();
                editor.scroll((i16::MIN / 2, 0));
                editor.move_cursor(CursorMove::Jump(row as u16, col as u16));
            }
        };
    }

    async fn handle_visual_mode_transition(&mut self, transition: VisualModeTransition) {
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
                self.context.notebook.update_yank();
            }
            DeleteSelection => {
                let editor = self.context.notebook.get_editor_mut();
                reselect_for_yank(editor);
                editor.cut();
                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteSelectionAndInsertMode => {
                let editor = self.context.notebook.get_editor_mut();
                reselect_for_yank(editor);
                editor.cut();
                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            SwitchCase => {
                let editor = self.context.notebook.get_editor_mut();
                switch_case(editor);

                self.context.notebook.mark_dirty();
            }
            ToLowercase => {
                let editor = self.context.notebook.get_editor_mut();
                let yank = editor.yank_text();
                reselect_for_yank(editor);
                editor.cut();

                let changed = editor.yank_text().as_str().to_lowercase();

                editor.insert_str(changed);
                editor.set_yank_text(yank);
            }
            ToUppercase => {
                let editor = self.context.notebook.get_editor_mut();
                let yank = editor.yank_text();
                reselect_for_yank(editor);
                editor.cut();

                let changed = editor.yank_text().as_str().to_uppercase();

                editor.insert_str(changed);
                editor.set_yank_text(yank);
            }
        }
    }

    pub(crate) async fn save(&mut self) {
        let mut transitions = vec![];

        for tab in self.context.notebook.tabs.iter() {
            if !tab.dirty {
                continue;
            }

            let event = NotebookEvent::UpdateNoteContent {
                note_id: tab.note.id.clone(),
                content: tab.editor.lines().join("\n"),
            }
            .into();

            let transition = self.glues.dispatch(event).await.log_unwrap();
            transitions.push(transition);
        }

        for transition in transitions {
            self.handle_transition(transition).await;
        }
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

fn cursor_move_back(editor: &TextArea, n: usize) -> CursorMove {
    let (row, col) = editor.cursor();
    if col < n {
        CursorMove::Head
    } else {
        CursorMove::Jump(row as u16, (col - n) as u16)
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

fn switch_case(editor: &mut TextArea) {
    let yank = editor.yank_text();
    reselect_for_yank(editor);
    editor.cut();

    let changed = editor
        .yank_text()
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect::<String>();

    editor.insert_str(changed);
    editor.set_yank_text(yank);
}
