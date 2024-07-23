mod add_directory;
mod add_note;
mod close_directory;
mod edit_mode;
mod initialize;
mod open_directory;
mod open_note;
mod remove_directory;
mod remove_note;
mod rename_directory;
mod rename_note;
mod select_note;
mod show_directory_actions;
mod show_note_actions;
mod view_mode;

use add_directory::add_directory;
use add_note::add_note;
use close_directory::close_directory;
use edit_mode::edit_mode;
use initialize::initialize;
use open_directory::open_directory;
use open_note::open_note;
use remove_directory::remove_directory;
use remove_note::remove_note;
use rename_directory::rename_directory;
use rename_note::rename_note;
use select_note::select_note;
use show_directory_actions::show_directory_actions;
use show_note_actions::show_note_actions;
use view_mode::view_mode;

use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::{Event, KeyEvent, Transition},
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
        Transition::OpenNote { note, content } => {
            open_note(siv, note, content);
        }
        Transition::EditMode => {
            edit_mode(siv);
        }
        Transition::ViewMode(note) => {
            view_mode(siv, note);
        }
        Transition::SelectNote(note) => {
            select_note(siv, note);
        }
        Transition::Inedible(Event::Key(KeyEvent::Esc)) => {
            siv.select_menubar();
        }
        _ => {
            log!("todo - unhandled event");
        }
    };

    update_statusbar(siv);
}

fn update_statusbar(siv: &mut Cursive) {
    let statusbar_node = Node::statusbar();

    let description = siv.glues().state.describe().log_unwrap();
    statusbar_node
        .description()
        .find(siv)
        .set_content(&description);

    let shortcuts = siv.glues().state.shortcuts().join(", ");
    statusbar_node.shortcuts().find(siv).set_content(&shortcuts);

    log!("[state] {description} / {shortcuts}");
}
