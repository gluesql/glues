mod editor;
mod note_tree;

use {
    crate::context::Context,
    ratatui::{
        Frame,
        layout::{
            Constraint::{Length, Percentage},
            Layout, Rect,
        },
    },
};

pub fn draw(frame: &mut Frame, area: Rect, context: &mut Context) {
    if !context.notebook.show_browser {
        editor::draw(frame, area, context);

        return;
    }

    let horizontal = Layout::horizontal([Length(45), Percentage(100)]);
    let [note_tree_area, editor_area] = horizontal.areas(area);

    note_tree::draw(frame, note_tree_area, &mut context.notebook);
    editor::draw(frame, editor_area, context);
}
