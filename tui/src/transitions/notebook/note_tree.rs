use {
    crate::{
        App,
        context::notebook::{TreeItem, TreeItemKind},
        logger::*,
    },
    glues_core::{
        Event, NotebookEvent,
        data::{Directory, Note},
        state::{GetInner, NotebookState},
        transition::{MoveModeTransition, NoteTreeTransition},
    },
};

impl App {
    pub(super) async fn handle_note_tree_transition(&mut self, transition: NoteTreeTransition) {
        let NotebookState { root, tabs, .. } = self.glues.state.get_inner().log_unwrap();

        match transition {
            NoteTreeTransition::OpenDirectory { id, .. } => {
                log!("Opening directory {id}");
                self.context.notebook.update_items(root);
            }
            NoteTreeTransition::CloseDirectory(id) => {
                log!("Closing directory {id}");
                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&id);
            }
            NoteTreeTransition::OpenNote { note, content, .. } => {
                self.context.notebook.open_note(note.id, content);
                self.context.notebook.tabs = tabs.clone();
                self.context.notebook.apply_yank();
            }
            NoteTreeTransition::RemoveNote {
                selected_directory, ..
            }
            | NoteTreeTransition::RemoveDirectory {
                selected_directory, ..
            } => {
                self.context.notebook.select_item(&selected_directory.id);
                self.context.notebook.update_items(root);
            }
            NoteTreeTransition::RenameDirectory(_) => {
                self.context.notebook.update_items(root);
                self.context.notebook.tabs = tabs.clone();
            }
            NoteTreeTransition::RenameNote(_) => {
                self.context.notebook.update_items(root);
                self.context.notebook.tabs = tabs.clone();
            }
            NoteTreeTransition::AddNote(Note {
                id,
                directory_id: parent_id,
                ..
            }) => {
                let transition = self
                    .glues
                    .dispatch(NotebookEvent::OpenDirectory(parent_id.clone()).into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;

                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&id);

                let transition = self
                    .glues
                    .dispatch(NotebookEvent::OpenNote.into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            NoteTreeTransition::AddDirectory(Directory { id, parent_id, .. }) => {
                let transition = self
                    .glues
                    .dispatch(NotebookEvent::OpenDirectory(parent_id.clone()).into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;

                let NotebookState { root, .. } = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.update_items(root);
                self.context.notebook.select_item(&id);
            }
            NoteTreeTransition::MoveMode(transition) => {
                self.handle_move_mode_transition(transition).await;
            }
            NoteTreeTransition::SelectNext(n) => {
                self.context.notebook.select_next(n);

                let selected = self.context.notebook.selected();
                let event = get_select_event(selected);
                self.glues.dispatch(event).await.log_unwrap();
            }
            NoteTreeTransition::SelectPrev(n) => {
                self.context.notebook.select_prev(n);

                let selected = self.context.notebook.selected();
                let event = get_select_event(selected);
                self.glues.dispatch(event).await.log_unwrap();
            }
            NoteTreeTransition::SelectFirst => {
                self.context.notebook.select_first();

                let selected = self.context.notebook.selected();
                let event = get_select_event(selected);
                self.glues.dispatch(event).await.log_unwrap();
            }
            NoteTreeTransition::SelectNextDirectory => {
                self.context.notebook.select_next_dir();

                let selected = self.context.notebook.selected();
                let event = get_select_event(selected);
                self.glues.dispatch(event).await.log_unwrap();
            }
            NoteTreeTransition::SelectPrevDirectory => {
                self.context.notebook.select_prev_dir();

                let selected = self.context.notebook.selected();
                let event = get_select_event(selected);
                self.glues.dispatch(event).await.log_unwrap();
            }
            NoteTreeTransition::SelectLast => {
                self.context.notebook.select_last();

                let selected = self.context.notebook.selected();
                let event = get_select_event(selected);
                self.glues.dispatch(event).await.log_unwrap();
            }
            NoteTreeTransition::ExpandWidth(n) => {
                let n = n.try_into().unwrap_or_default();
                let width = self.context.notebook.tree_width.saturating_add(n);

                self.context.notebook.tree_width = width;
            }
            NoteTreeTransition::ShrinkWidth(n) => {
                let n = n.try_into().unwrap_or_default();
                let width = self.context.notebook.tree_width.saturating_sub(n);
                let width = if width < 11 { 11 } else { width };

                self.context.notebook.tree_width = width;
            }
            NoteTreeTransition::GatewayMode
            | NoteTreeTransition::ShowNoteActionsDialog(_)
            | NoteTreeTransition::ShowDirectoryActionsDialog(_) => {}
        }

        fn get_select_event(selected: &TreeItem) -> Event {
            match selected {
                TreeItem {
                    kind: TreeItemKind::Directory { directory, .. },
                    ..
                } => NotebookEvent::SelectDirectory(directory.clone()).into(),
                TreeItem {
                    kind: TreeItemKind::Note { note },
                    ..
                } => NotebookEvent::SelectNote(note.clone()).into(),
            }
        }
    }

    async fn handle_move_mode_transition(&mut self, transition: MoveModeTransition) {
        use MoveModeTransition::*;

        match transition {
            Enter => {
                let state: &NotebookState = self.glues.state.get_inner().log_unwrap();

                self.context.notebook.update_items(&state.root);
                self.context.notebook.select_prev(1);
            }
            SelectNext => {
                self.context.notebook.select_next(1);
            }
            SelectPrev => {
                self.context.notebook.select_prev(1);
            }
            SelectLast => {
                self.context.notebook.select_last();
            }
            RequestCommit => {
                let is_directory = self
                    .context
                    .notebook
                    .tree_items
                    .iter()
                    .find(|item| item.target)
                    .log_expect("No target selected")
                    .is_directory();
                let event = match self.context.notebook.selected() {
                    TreeItem {
                        kind: TreeItemKind::Directory { directory, .. },
                        ..
                    } => {
                        if is_directory {
                            NotebookEvent::MoveDirectory(directory.id.clone()).into()
                        } else {
                            NotebookEvent::MoveNote(directory.id.clone()).into()
                        }
                    }
                    _ => {
                        let message = format!(
                            "Error - Cannot move {} to note",
                            if is_directory { "directory" } else { "note" }
                        );
                        log!("{message}");
                        self.context.alert = Some(message);

                        return;
                    }
                };

                let transition = self.glues.dispatch(event).await.log_unwrap();
                self.handle_transition(transition).await;
            }
            Commit => {
                let state: &NotebookState = self.glues.state.get_inner().log_unwrap();
                let id = state.get_selected_id().log_unwrap();

                self.context.notebook.update_items(&state.root);
                self.context.notebook.select_item(id);
                self.context.notebook.tabs = state.tabs.clone();
            }
            Cancel => {
                let state: &NotebookState = self.glues.state.get_inner().log_unwrap();
                let id = state.get_selected_id().log_unwrap();

                self.context.notebook.update_items(&state.root);
                self.context.notebook.select_item(id);
            }
        }
    }
}
