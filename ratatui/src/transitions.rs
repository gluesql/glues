use {
    super::{logger::*, App},
    glues_core::{
        state::{GetInner, NotebookState},
        transition::{EntryTransition, NotebookTransition, Transition},
    },
};

impl App {
    pub(super) fn handle_transition(&mut self, transition: Transition) {
        match transition {
            Transition::Entry(transition) => {
                self.handle_entry_transition(transition);
            }
            Transition::Notebook(transition) => {
                self.handle_notebook_transition(transition);
            }
            Transition::Log(message) => {
                log!("{message}");
            }
            Transition::Error(message) => {
                log!("[Err] {message}");
                // todo! - show dialog with error message
            }
        }
    }

    pub(super) fn handle_entry_transition(&mut self, transition: EntryTransition) {
        match transition {
            EntryTransition::OpenNotebook => {
                log!("Opening notebook");

                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();
                self.context.notebook.update_items(root);
            }
            EntryTransition::Inedible(event) => {
                log!("Inedible event: {event}");
            }
            EntryTransition::None => {}
        }
    }

    pub(super) fn handle_notebook_transition(&mut self, transition: NotebookTransition) {
        match transition {
            NotebookTransition::OpenDirectory { id, .. } => {
                log!("Opening directory {id}");
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();
                self.context.notebook.update_items(root);
            }
            NotebookTransition::CloseDirectory(id) => {
                log!("Closing directory {id}");
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();
                self.context.notebook.update_items(root);
            }
            NotebookTransition::OpenNote { note, content } => {
                self.context.notebook.open_note(note, content);
            }
            NotebookTransition::ViewMode(_note) => {}
            NotebookTransition::EditMode => {}
            _ => {}
        }
    }
}
