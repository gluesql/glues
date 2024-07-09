mod add_directory;
mod add_note;
mod close_directory;
mod initialize;
mod open_directory;
mod remove_directory;
mod remove_note;
mod rename_directory;
mod rename_note;
mod show_directory_actions;
mod show_note_actions;

use add_directory::add_directory;
use add_note::add_note;
use close_directory::close_directory;
use initialize::initialize;
use open_directory::open_directory;
use remove_directory::remove_directory;
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
        Transition::Initialize => initialize(siv),
        Transition::ShowNoteActionsDialog(note) => show_note_actions(siv, note),
        Transition::ShowDirectoryActionsDialog(directory) => show_directory_actions(siv, directory),
        Transition::RenameNote(note) => rename_note(siv, note),
        Transition::RemoveNote(note) => remove_note(siv, note),
        Transition::AddNote(note) => add_note(siv, note),
        Transition::RenameDirectory(directory) => rename_directory(siv, directory),
        Transition::RemoveDirectory(directory) => remove_directory(siv, directory),
        Transition::AddDirectory(directory) => add_directory(siv, directory),
        Transition::OpenDirectory {
            id,
            notes,
            directories,
        } => {
            open_directory(siv, id, notes, directories);
        }
        Transition::CloseDirectory(id) => close_directory(siv, id),
        _ => {
            log("todo");
        }
    };

    actions::update_statusbar(siv);
}
