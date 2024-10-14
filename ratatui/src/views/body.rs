mod entry;
mod notebook;

use {
    crate::{context::ContextState, Context},
    ratatui::{layout::Rect, Frame},
};

pub fn draw(frame: &mut Frame, area: Rect, context: &mut Context) {
    match context.state {
        ContextState::Entry => entry::draw(frame, area, &mut context.entry),
        ContextState::Notebook => notebook::draw(frame, area, &mut context.notebook),
    }
}
