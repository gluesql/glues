mod alert;
mod confirm;
mod directory_actions;
mod editor_keymap;
mod help;
mod keymap;
mod note_actions;
mod prompt;
mod vim_keymap;

use {
    crate::{
        Context,
        context::{self},
    },
    glues_core::state::State,
    ratatui::Frame,
};

pub fn draw(frame: &mut Frame, state: &State, context: &mut Context) {
    if context.keymap {
        keymap::draw(frame, context, state.keymap().as_slice());
    }

    if let Some(kind) = context.vim_keymap {
        vim_keymap::draw(frame, kind);
        return;
    } else if context.editor_keymap {
        editor_keymap::draw(frame);
        return;
    } else if context.help {
        help::draw(frame);
        return;
    } else if context.alert.is_some() {
        alert::draw(frame, context);
        return;
    } else if context.confirm.is_some() {
        confirm::draw(frame, context);
        return;
    } else if context.prompt.is_some() {
        prompt::draw(frame, context);
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
