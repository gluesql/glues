use {
    super::textarea::{set_selection, switch_case},
    crate::{App, logger::*},
    edtui::{
        EditorMode, Index2,
        actions::{
            AppendNewline, ChangeInnerWord, CopyLine, CopySelection, DeleteLine, DeleteSelection,
            DeleteToFirstCharOfLine, InsertNewline, MoveBackward, MoveDown, MoveForward,
            MoveToEndOfLine, MoveToFirst, MoveToStartOfLine, MoveUp, MoveWordBackward,
            MoveWordForward, MoveWordForwardToEndOfWord, RemoveChar, SwitchMode,
            delete::DeleteToEndOfLine, motion::MoveToFirstRow, motion::MoveToLastRow,
        },
    },
    glues_core::{
        state::{GetInner, NotebookState},
        transition::NormalModeTransition,
    },
};

impl App {
    pub(super) async fn handle_normal_mode_transition(&mut self, transition: NormalModeTransition) {
        use NormalModeTransition::*;
        let NotebookState { root, tabs, .. } = self.glues.state.get_inner().log_unwrap();

        match transition {
            IdleMode => {
                let editor = self.context.notebook.get_editor_mut();
                editor.selection = None;
                editor.execute(SwitchMode(EditorMode::Normal));
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
            ToggleSyntaxHighlight => {
                self.context.notebook.show_syntax_highlight =
                    !self.context.notebook.show_syntax_highlight;
            }
            MoveCursorDown(n) => {
                self.context.notebook.get_editor_mut().execute(MoveDown(n));
            }
            MoveCursorUp(n) => {
                self.context.notebook.get_editor_mut().execute(MoveUp(n));
            }
            MoveCursorBack(n) => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveBackward(n));
            }
            MoveCursorForward(n) => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveForward(n));
            }
            MoveCursorWordForward(n) => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveWordForward(n));
            }
            MoveCursorWordEnd(n) => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveWordForwardToEndOfWord(n));
            }
            MoveCursorWordBack(n) => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveWordBackward(n));
            }
            MoveCursorLineStart => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveToStartOfLine());
            }
            MoveCursorLineEnd => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveToEndOfLine());
            }
            MoveCursorLineNonEmptyStart => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveToFirst());
            }
            MoveCursorTop => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveToFirstRow());
            }
            MoveCursorBottom => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveToLastRow());
            }
            MoveCursorToLine(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let target_row = n
                    .saturating_sub(1)
                    .min(editor.lines.len().saturating_sub(1));
                editor.cursor = Index2::new(target_row, 0);
                editor.execute(MoveToFirst());
            }
            InsertNewLineBelow => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(AppendNewline(1));
                editor.execute(SwitchMode(EditorMode::Insert));
            }
            InsertNewLineAbove => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(InsertNewline(1));
                editor.execute(SwitchMode(EditorMode::Insert));
            }
            InsertAtCursor => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(SwitchMode(EditorMode::Insert));
            }
            InsertAtLineStart => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(SwitchMode(EditorMode::Insert));
                editor.execute(MoveToStartOfLine());
            }
            InsertAfterCursor => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(SwitchMode(EditorMode::Insert));
                editor.execute(MoveForward(1));
            }
            InsertAtLineEnd => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(SwitchMode(EditorMode::Insert));
                editor.execute(MoveToEndOfLine());
            }
            DeleteChars(n) => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(RemoveChar(n));
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteCharsBack(n) => {
                let editor = self.context.notebook.get_editor_mut();
                for _ in 0..n {
                    editor.execute(MoveBackward(1));
                    editor.execute(RemoveChar(1));
                }
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            Paste => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(edtui::actions::Paste);
                self.context.notebook.mark_dirty();
            }
            Undo => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(edtui::actions::Undo);
                self.context.notebook.mark_dirty();
            }
            Redo => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(edtui::actions::Redo);
                self.context.notebook.mark_dirty();
            }
            YankLines(n) => {
                let editor = self.context.notebook.get_editor_mut();
                if n == 1 {
                    editor.execute(CopyLine);
                } else {
                    let cursor = editor.cursor;
                    // Select from start of current line to end of (n-1)th line below
                    let start = Index2::new(cursor.row, 0);
                    let end_row = (cursor.row + n - 1).min(editor.lines.len().saturating_sub(1));
                    let end_col = editor.lines.len_col(end_row).unwrap_or(0).saturating_sub(1);
                    let end = Index2::new(end_row, end_col);
                    set_selection(editor, start, end);
                    editor.execute(CopySelection);
                    // Prepend \n so paste knows it's a line-mode yank
                    let clip = self.context.notebook.get_clipboard();
                    let text = clip.get_text();
                    clip.set_text(String::from('\n') + &text);
                    self.context.notebook.get_editor_mut().cursor = cursor;
                }
                self.context.notebook.line_yanked = true;
                self.context.notebook.update_yank();
            }
            DeleteLines(n) => {
                let editor = self.context.notebook.get_editor_mut();
                for _ in 0..n {
                    editor.execute(DeleteLine(1));
                }
                editor.execute(MoveToFirst());
                self.context.notebook.line_yanked = true;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteLinesUp(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let (row, _) = (editor.cursor.row, editor.cursor.col);
                let start_row = row.saturating_sub(n - 1);
                editor.cursor = Index2::new(start_row, 0);
                for _ in 0..n {
                    editor.execute(DeleteLine(1));
                }
                editor.execute(MoveToFirst());
                self.context.notebook.line_yanked = true;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteLinesAndInsert(n) => {
                let editor = self.context.notebook.get_editor_mut();
                // Delete n lines but keep cursor on the empty line for insert mode
                editor.execute(MoveToStartOfLine());
                // Select from head to end of nth line
                let cursor = editor.cursor;
                let end_row = (cursor.row + n - 1).min(editor.lines.len().saturating_sub(1));
                let end_col = editor.lines.len_col(end_row).unwrap_or(0);
                let end = Index2::new(end_row, end_col.saturating_sub(1));
                set_selection(editor, Index2::new(cursor.row, 0), end);
                editor.execute(DeleteSelection);
                editor.execute(SwitchMode(EditorMode::Insert));
                self.context.notebook.line_yanked = true;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteInsideWord(n) => {
                let editor = self.context.notebook.get_editor_mut();
                for _ in 0..n {
                    editor.execute(ChangeInnerWord);
                    editor.execute(SwitchMode(EditorMode::Normal));
                }
                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteWordEnd(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let start = editor.cursor;
                editor.execute(MoveWordForwardToEndOfWord(n));
                editor.execute(MoveForward(1));
                let end = editor.cursor;
                set_selection(editor, start, end);
                editor.execute(DeleteSelection);
                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteWordBack(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let end = editor.cursor;
                editor.execute(MoveWordBackward(n));
                let start = editor.cursor;
                set_selection(editor, start, end);
                editor.execute(DeleteSelection);
                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteLineStart => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(DeleteToFirstCharOfLine);
                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteLineEnd(n) => {
                let editor = self.context.notebook.get_editor_mut();
                if n == 1 {
                    editor.execute(DeleteToEndOfLine);
                } else {
                    let start = editor.cursor;
                    editor.execute(MoveDown(n - 1));
                    editor.execute(MoveToEndOfLine());
                    let end = editor.cursor;
                    set_selection(editor, start, end);
                    editor.execute(DeleteSelection);
                }
                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            SwitchCase => {
                let editor = self.context.notebook.get_editor_mut();
                switch_case(editor);
                self.context.notebook.mark_dirty();
            }
            ScrollCenter => {
                self.context.notebook.pending_scroll =
                    Some(crate::context::notebook::ScrollRequest::Center);
            }
            ScrollTop => {
                self.context.notebook.pending_scroll =
                    Some(crate::context::notebook::ScrollRequest::Top);
            }
            ScrollBottom => {
                self.context.notebook.pending_scroll =
                    Some(crate::context::notebook::ScrollRequest::Bottom);
            }
        };
    }
}
