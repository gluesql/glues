mod entry;
mod keymap;
mod notebook;

use {
    super::App, async_recursion::async_recursion, glues_core::transition::Transition,
    std::time::SystemTime,
};

#[cfg(not(target_arch = "wasm32"))]
use glues_core::transition::{MoveModeTransition, NoteTreeTransition, NotebookTransition};

impl App {
    #[async_recursion(?Send)]
    pub(super) async fn handle_transition(&mut self, transition: Transition) {
        #[cfg(not(target_arch = "wasm32"))]
        let should_sync = transition_requires_sync(&transition);

        match transition {
            Transition::Keymap(transition) => {
                self.handle_keymap_transition(transition).await;
            }
            Transition::Entry(transition) => {
                self.handle_entry_transition(transition).await;
            }
            Transition::Notebook(transition) => {
                self.handle_notebook_transition(transition).await;
            }
            Transition::Log(message) => {
                log!("{message}");
                self.context.last_log = Some((message, SystemTime::now()));
            }
            Transition::Error(message) => {
                log!("[Err] {message}");
                self.context.alert = Some(message);
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        if should_sync {
            self.maybe_schedule_sync();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn transition_requires_sync(transition: &Transition) -> bool {
    matches!(
        transition,
        Transition::Notebook(NotebookTransition::NoteTree(
            NoteTreeTransition::RenameNote(_)
                | NoteTreeTransition::RenameDirectory(_)
                | NoteTreeTransition::RemoveNote { .. }
                | NoteTreeTransition::RemoveDirectory { .. }
                | NoteTreeTransition::AddNote(_)
                | NoteTreeTransition::AddDirectory(_)
                | NoteTreeTransition::MoveMode(MoveModeTransition::Commit)
        )) | Transition::Notebook(NotebookTransition::UpdateNoteContent(_))
    )
}
