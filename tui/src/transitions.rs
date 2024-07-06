mod show_directory_actions;
mod show_note_actions;

pub use show_directory_actions::show_directory_actions;
pub use show_note_actions::show_note_actions;

use {
    crate::{actions, traits::*},
    cursive::Cursive,
    glues_core::{Event, Transition},
};

pub fn handle_event(siv: &mut Cursive, event: Event) {
    let transition = siv.glues().dispatch(event).log_unwrap();

    match transition {
        Transition::ShowNoteActionsDialog(payload) => {
            show_note_actions(siv, payload.note);
        }
        Transition::ShowDirectoryActionsDialog(payload) => {
            show_directory_actions(siv, payload.directory);
        }
        _ => {
            log("todo");
        }
    };

    actions::update_statusbar(siv);
}
