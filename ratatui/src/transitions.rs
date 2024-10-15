use {
    super::{
        context::{self, ContextState},
        logger::*,
        App,
    },
    glues_core::{
        data::{Directory, Note},
        state::{GetInner, NotebookState},
        transition::{EntryTransition, NotebookTransition, Transition},
        NotebookEvent,
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
                self.context.alert = Some(message);
            }
        }
    }

    pub(super) fn handle_entry_transition(&mut self, transition: EntryTransition) {
        match transition {
            EntryTransition::OpenNotebook => {
                log!("Opening notebook");

                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();
                self.context.state = ContextState::Notebook;
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
                self.context.notebook.select_item(&id);
            }
            NotebookTransition::OpenNote { note, content } => {
                self.context.notebook.open_note(note, content);
            }
            NotebookTransition::ViewMode(_note) => {
                let content: String = self.context.notebook.editor_state.lines.clone().into();
                let event = NotebookEvent::UpdateNoteContent(content).into();

                self.glues.dispatch(event).log_unwrap();
            }
            NotebookTransition::EditMode => {}
            NotebookTransition::RemoveNote {
                selected_directory, ..
            }
            | NotebookTransition::RemoveDirectory {
                selected_directory, ..
            } => {
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.select_item(&selected_directory.id);
                self.context.notebook.state = context::notebook::ContextState::NoteTreeBrowsing;
                self.context.notebook.update_items(root);
            }
            NotebookTransition::RenameNote(_) | NotebookTransition::RenameDirectory(_) => {
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.state = context::notebook::ContextState::NoteTreeBrowsing;
                self.context.notebook.update_items(root);
            }
            NotebookTransition::AddNote(Note {
                id,
                directory_id: parent_id,
                ..
            })
            | NotebookTransition::AddDirectory(Directory { id, parent_id, .. }) => {
                self.glues
                    .dispatch(NotebookEvent::OpenDirectory(parent_id.clone()).into())
                    .log_unwrap();
                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.state = context::notebook::ContextState::NoteTreeBrowsing;
                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&id);
            }
            _ => {}
        }
    }
}
