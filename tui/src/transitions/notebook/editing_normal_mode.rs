use {
    super::textarea::*,
    crate::{logger::*, App},
    glues_core::{
        state::{GetInner, NotebookState},
        transition::NormalModeTransition,
    },
    tui_textarea::{CursorMove, Scrolling},
};

impl App {
    pub(super) async fn handle_normal_mode_transition(&mut self, transition: NormalModeTransition) {
        use NormalModeTransition::*;
        let NotebookState { root, tabs, .. } = self.glues.state.get_inner().log_unwrap();

        match transition {
            IdleMode => {
                self.context.notebook.get_editor_mut().cancel_selection();
            }
            ToggleMode | ToggleTabCloseMode | NumberingMode | GatewayMode | YankMode
            | DeleteMode | DeleteInsideMode | ChangeMode | ChangeInsideMode | ScrollMode => {}
            NextTab(note_id) | PrevTab(note_id) => {
                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&note_id);
                self.context.notebook.apply_yank();
            }
            MoveTabNext(_) | MoveTabPrev(_) | CloseRightTabs(_) | CloseLeftTabs(_) => {
                self.context.notebook.tabs = tabs.clone();
            }
            CloseTab(note_id) => {
                self.context.notebook.tabs = tabs.clone();
                self.save().await;
                self.context.notebook.editors.remove(&note_id);

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
                let editor = self.context.notebook.get_editor_mut();
                let cursor_move = cursor_move_forward(editor, 1);

                editor.move_cursor(cursor_move);
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
}
