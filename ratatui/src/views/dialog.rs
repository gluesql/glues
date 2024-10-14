mod alert;
mod confirm;
mod directory_actions;
mod note_actions;

use {
    crate::{
        context::{self},
        Context,
    },
    ratatui::Frame,
};

pub fn draw(frame: &mut Frame, context: &mut Context) {
    if context.alert.is_some() {
        alert::draw(frame, context);
        return;
    } else if context.confirm.is_some() {
        confirm::draw(frame, context);
        return;
    }

    match context.notebook.state {
        context::notebook::ContextState::NoteActionsDialog => {
            note_actions::draw(frame, &mut context.notebook);
        }
        context::notebook::ContextState::DirectoryActionsDialog => {
            directory_actions::draw(frame, &mut context.notebook);
        }
        _ => {}
    }
}
