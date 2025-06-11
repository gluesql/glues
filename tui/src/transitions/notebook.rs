mod editing_normal_mode;
mod editing_visual_mode;
mod note_tree;
mod textarea;

use {
    crate::{
        App,
        context::{self},
        logger::*,
    },
    glues_core::{
        NotebookEvent,
        state::{
            GetInner, NotebookState,
            notebook::{InnerState, NoteTreeState, VimNormalState},
        },
        transition::NotebookTransition,
    },
};

impl App {
    pub(super) async fn handle_notebook_transition(&mut self, transition: NotebookTransition) {
        use context::notebook::ContextState;

        let NotebookState {
            root,
            inner_state,
            tab_index,
            ..
        } = self.glues.state.get_inner().log_unwrap();
        let new_state = match inner_state {
            InnerState::NoteTree(
                NoteTreeState::NoteSelected | NoteTreeState::DirectorySelected,
            ) => ContextState::NoteTreeBrowsing,
            InnerState::NoteTree(NoteTreeState::Numbering(_)) => ContextState::NoteTreeNumbering,
            InnerState::NoteTree(NoteTreeState::GatewayMode) => ContextState::NoteTreeGateway,
            InnerState::NoteTree(NoteTreeState::NoteMoreActions) => ContextState::NoteActionsDialog,
            InnerState::NoteTree(NoteTreeState::DirectoryMoreActions) => {
                ContextState::DirectoryActionsDialog
            }
            InnerState::NoteTree(NoteTreeState::MoveMode) => ContextState::MoveMode,
            InnerState::EditingNormalMode(VimNormalState::Idle) => {
                ContextState::EditorNormalMode { idle: true }
            }
            InnerState::EditingNormalMode(_) => ContextState::EditorNormalMode { idle: false },
            InnerState::EditingVisualMode(_) => ContextState::EditorVisualMode,
            InnerState::EditingInsertMode => ContextState::EditorInsertMode,
        };

        if self.context.notebook.state != new_state {
            self.context.notebook.state = new_state;
        }

        if &self.context.notebook.tab_index != tab_index {
            self.context.notebook.tab_index = *tab_index;
        }

        match transition {
            NotebookTransition::ShowVimKeymap(kind) => {
                self.context.keymap_scroll = 0;
                self.context.vim_keymap = Some(kind);
            }
            NotebookTransition::ViewMode(_note) => {
                self.context.notebook.mark_dirty();
            }
            NotebookTransition::UpdateNoteContent(note_id) => {
                self.context.notebook.mark_clean(&note_id);
            }
            NotebookTransition::BrowseNoteTree => {}
            NotebookTransition::FocusEditor => {
                let note_id = self
                    .context
                    .notebook
                    .get_opened_note()
                    .log_expect("No note opened")
                    .id
                    .clone();

                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&note_id);
            }
            NotebookTransition::NoteTree(transition) => {
                self.handle_note_tree_transition(transition).await;
            }
            NotebookTransition::EditingNormalMode(transition) => {
                self.handle_normal_mode_transition(transition).await;
            }
            NotebookTransition::EditingVisualMode(transition) => {
                self.handle_visual_mode_transition(transition).await;
            }
            NotebookTransition::Alert(message) => {
                log!("[Alert] {message}");
                self.context.alert = Some(message);
            }
            NotebookTransition::Inedible(_) | NotebookTransition::None => {}
        }
    }

    pub(crate) async fn save(&mut self) {
        let mut transitions = vec![];

        for (note_id, item) in self.context.notebook.editors.iter() {
            if !item.dirty {
                continue;
            }

            let event = NotebookEvent::UpdateNoteContent {
                note_id: note_id.clone(),
                content: item.editor.lines().join("\n"),
            }
            .into();

            let transition = self.glues.dispatch(event).await.log_unwrap();
            transitions.push(transition);
        }

        for transition in transitions {
            self.handle_transition(transition).await;
        }
    }
}
