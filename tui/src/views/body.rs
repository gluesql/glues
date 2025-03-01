mod entry;
mod notebook;

use {
    crate::{Context, context::ContextState},
    ratatui::{Frame, layout::Rect},
};

pub fn draw(frame: &mut Frame, area: Rect, context: &mut Context) {
    match context.state {
        ContextState::Entry => entry::draw(frame, area, &mut context.entry),
        ContextState::Notebook => notebook::draw(frame, area, context),
    }
}
