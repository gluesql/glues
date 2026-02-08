use {
    super::textarea::{
        move_word_backward, move_word_forward, move_word_forward_to_end, switch_case_selection,
        transform_selection,
    },
    crate::App,
    edtui::{
        EditorMode,
        actions::{
            ChangeSelection, CopySelection, MoveBackward, MoveDown, MoveForward, MoveToEndOfLine,
            MoveToFirst, MoveToStartOfLine, MoveUp, SwitchMode, motion::MoveToFirstRow,
            motion::MoveToLastRow,
        },
    },
    glues_core::transition::VisualModeTransition,
};

impl App {
    pub(super) async fn handle_visual_mode_transition(&mut self, transition: VisualModeTransition) {
        use VisualModeTransition::*;

        match transition {
            IdleMode => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(SwitchMode(EditorMode::Visual));
            }
            NumberingMode | GatewayMode => {}
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
                move_word_forward(self.context.notebook.get_editor_mut(), n);
            }
            MoveCursorWordEnd(n) => {
                move_word_forward_to_end(self.context.notebook.get_editor_mut(), n);
            }
            MoveCursorWordBack(n) => {
                move_word_backward(self.context.notebook.get_editor_mut(), n);
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
            MoveCursorBottom => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveToLastRow());
            }
            MoveCursorTop => {
                self.context
                    .notebook
                    .get_editor_mut()
                    .execute(MoveToFirstRow());
            }
            MoveCursorToLine(n) => {
                let editor = self.context.notebook.get_editor_mut();
                let target_row = n.saturating_sub(1);
                // In visual mode, moving cursor updates selection automatically via edtui
                if target_row < editor.cursor.row {
                    editor.execute(MoveUp(editor.cursor.row - target_row));
                } else if target_row > editor.cursor.row {
                    editor.execute(MoveDown(target_row - editor.cursor.row));
                }
                editor.execute(MoveToFirst());
            }
            YankSelection => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(CopySelection);
                self.context.notebook.line_yanked = false;
                self.context.notebook.update_yank();
            }
            DeleteSelection => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(edtui::actions::DeleteSelection);
                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            DeleteSelectionAndInsertMode => {
                let editor = self.context.notebook.get_editor_mut();
                editor.execute(ChangeSelection);
                editor.execute(SwitchMode(EditorMode::Insert));
                self.context.notebook.line_yanked = false;
                self.context.notebook.mark_dirty();
                self.context.notebook.update_yank();
            }
            SwitchCase => {
                let editor = self.context.notebook.get_editor_mut();
                switch_case_selection(editor);
                self.context.notebook.mark_dirty();
            }
            ToLowercase => {
                let editor = self.context.notebook.get_editor_mut();
                transform_selection(editor, |c| c.to_lowercase().next().unwrap_or(c));
                self.context.notebook.mark_dirty();
            }
            ToUppercase => {
                let editor = self.context.notebook.get_editor_mut();
                transform_selection(editor, |c| c.to_uppercase().next().unwrap_or(c));
                self.context.notebook.mark_dirty();
            }
        }
    }
}
