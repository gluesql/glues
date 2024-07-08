mod close_directory;
mod initialize;
mod open_directory;
mod remove_note;
mod rename_directory;
mod rename_note;
mod show_directory_actions;
mod show_note_actions;

use close_directory::close_directory;
use initialize::initialize;
use open_directory::open_directory;
use remove_note::remove_note;
use rename_directory::rename_directory;
use rename_note::rename_note;
use show_directory_actions::show_directory_actions;
use show_note_actions::show_note_actions;

use {
    crate::{actions, traits::*},
    cursive::Cursive,
    glues_core::{Event, Transition},
};

pub fn handle_event(siv: &mut Cursive, event: Event) {
    let transition = siv.glues().dispatch(event).log_unwrap();

    match transition {
        Transition::Initialize => {
            initialize(siv);
        }
        Transition::ShowNoteActionsDialog(payload) => {
            show_note_actions(siv, payload.note);
        }
        Transition::ShowDirectoryActionsDialog(payload) => {
            show_directory_actions(siv, payload.directory);
        }
        Transition::RenameNote { id, name } => {
            rename_note(siv, id, name);
        }
        Transition::RemoveNote(note) => remove_note(siv, note),
        Transition::RenameDirectory { id, name } => {
            rename_directory(siv, id, name);
        }
        Transition::OpenDirectory {
            id,
            notes,
            directories,
        } => {
            open_directory(siv, id, notes, directories);
        }
        Transition::CloseDirectory(id) => {
            close_directory(siv, id);
        }
        _ => {
            log("todo");
        }
    };

    actions::update_statusbar(siv);
}
