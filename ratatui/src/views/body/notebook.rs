mod editor;
mod note_tree;

use {
    crate::context::NotebookContext,
    glues_core::state::NotebookState,
    ratatui::{
        layout::{
            Constraint::{Length, Percentage},
            Layout, Rect,
        },
        Frame,
    },
};

pub fn draw(frame: &mut Frame, area: Rect, state: &NotebookState, context: &mut NotebookContext) {
    let horizontal = Layout::horizontal([Length(45), Percentage(100)]);
    let [note_tree_area, editor_area] = horizontal.areas(area);

    note_tree::draw(frame, note_tree_area, state, context);
    editor::draw(frame, editor_area, state, context);
}
