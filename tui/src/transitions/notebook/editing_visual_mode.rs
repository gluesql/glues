use {
    super::textarea::*, crate::App, glues_core::transition::VisualModeTransition,
    tui_textarea::CursorMove,
};

impl App {
    pub(super) async fn handle_visual_mode_transition(&mut self, transition: VisualModeTransition) {
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
}
