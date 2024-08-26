mod entry;
mod notebook;

use {
    crate::{traits::*, Node},
    cursive::Cursive,
    glues_core::{EntryTransition, Event, KeyEvent, NotebookTransition, Transition},
};

pub fn handle_event(siv: &mut Cursive, event: Event) {
    let transition = siv.glues().dispatch(event).log_unwrap();

    handle_transition(siv, transition)
}

pub fn handle_queue(siv: &mut Cursive) {
    let mut transitions = Vec::new();
    {
        let mut queue = siv.glues().transition_queue.lock().log_unwrap();

        while let Some(transition) = queue.pop_front() {
            transitions.push(transition);
        }
    }

    for transition in transitions {
        handle_transition(siv, transition);
    }
}

fn handle_transition(siv: &mut Cursive, transition: Transition) {
    match transition {
        Transition::Entry(transition) => handle_entry_transition(siv, transition),
        Transition::Notebook(transition) => handle_notebook_transition(siv, transition),

        Transition::Log(message) => {
            log!("{message}");
            return;
        }
        Transition::Error(message) => {
            log!("[Err] {message}");

            siv.alert(format!("[ERROR] {message}"), |_| {});
            return;
        }
    };

    update_statusbar(siv);
}

fn handle_entry_transition(siv: &mut Cursive, transition: EntryTransition) {
    use entry::*;

    match transition {
        EntryTransition::OpenNotebook => open_notebook(siv),
        EntryTransition::Inedible(Event::Key(KeyEvent::Esc)) => {
            log!("[EntryTransition] inedible event - ESC");
        }
        _ => {
            log!("[EntryTransition] unhandled - {transition}");
        }
    }
}

fn handle_notebook_transition(siv: &mut Cursive, transition: NotebookTransition) {
    use notebook::*;

    match transition {
        NotebookTransition::ShowNoteActionsDialog(note) => show_note_actions(siv, note),
        NotebookTransition::ShowDirectoryActionsDialog(directory) => {
            show_directory_actions(siv, directory)
        }
        NotebookTransition::RenameNote(note) => rename_note(siv, note),
        NotebookTransition::RemoveNote {
            note,
            selected_directory,
        } => remove_note(siv, note, selected_directory),
        NotebookTransition::AddNote(note) => add_note(siv, note),
        NotebookTransition::RenameDirectory(directory) => rename_directory(siv, directory),
        NotebookTransition::RemoveDirectory {
            directory,
            selected_directory,
        } => remove_directory(siv, directory, selected_directory),
        NotebookTransition::AddDirectory(directory) => add_directory(siv, directory),
        NotebookTransition::OpenDirectory {
            id,
            notes,
            directories,
        } => {
            open_directory(siv, id, notes, directories);
        }
        NotebookTransition::CloseDirectory {
            directory_id,
            by_note,
        } => close_directory(siv, directory_id, by_note),
        NotebookTransition::OpenNote { note, content } => {
            open_note(siv, note, content);
        }
        NotebookTransition::EditMode => {
            edit_mode(siv);
        }
        NotebookTransition::ViewMode(note) => {
            view_mode(siv, note);
        }
        NotebookTransition::SelectNote(note) => select_note(siv, note),
        NotebookTransition::SelectDirectory(directory) => select_directory(siv, directory),
        NotebookTransition::UpdateNoteContent => {
            update_note_content(siv);
        }
        NotebookTransition::ShowEntryDialog => {
            show_entry_dialog(siv);
        }
        NotebookTransition::Inedible(Event::Key(KeyEvent::Esc)) => {
            log!("[NotebookTransition] inedible event - ESC");
        }
        _ => {
            log!("[NotebookTransition] unhandled - {transition}");
        }
    }
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

    log!("[state] {description}");
}
