mod entry;
mod notebook;

use {
    crate::Context,
    glues_core::state::State,
    ratatui::{layout::Rect, Frame},
};

pub fn draw(frame: &mut Frame, area: Rect, state: &State, context: &mut Context) {
    match state {
        State::EntryState(state) => entry::draw(frame, area, state, &mut context.entry),
        State::NotebookState(state) => notebook::draw(frame, area, state, &mut context.notebook),
    }
}
