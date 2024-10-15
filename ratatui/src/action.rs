use {
    super::{context::ContextPrompt, logger::*, App},
    glues_core::{EntryEvent, Event, KeyEvent, NotebookEvent},
    ratatui::{
        crossterm::event::{Event as Input, KeyCode},
        text::Line,
    },
};

#[derive(Clone)]
pub enum Action {
    Tui(TuiAction),
    Dispatch(Event),
    PassThrough,
    None,
}

#[derive(Clone)]
pub enum TuiAction {
    Confirm {
        message: String,
        action: Box<Action>,
    },
    Prompt {
        message: Vec<Line<'static>>,
        action: Box<Action>,
    },
    Quit,

    RenameNote,
    RemoveNote,
    AddNote,
    AddDirectory,
    RenameDirectory,
    RemoveDirectory,
}

impl From<TuiAction> for Action {
    fn from(action: TuiAction) -> Self {
        Self::Tui(action)
    }
}

impl From<EntryEvent> for Action {
    fn from(event: EntryEvent) -> Self {
        Self::Dispatch(event.into())
    }
}

impl App {
    pub(super) fn handle_action(&mut self, action: Action, input: Input) {
        match action {
            Action::Tui(TuiAction::Quit) => {}
            Action::Tui(TuiAction::Confirm { message, action }) => {
                self.context.confirm = Some((message, *action));
            }
            Action::Tui(TuiAction::Prompt { message, action }) => {
                self.context.prompt = Some(ContextPrompt::new(message, *action));
            }
            Action::Tui(TuiAction::RenameNote) => {
                let new_name = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if new_name.is_empty() {
                    self.context.alert = Some("Note name cannot be empty".to_string());
                    return;
                }

                let transition = self
                    .glues
                    .dispatch(NotebookEvent::RenameNote(new_name).into())
                    .log_unwrap();
                self.handle_transition(transition);
            }
            Action::Tui(TuiAction::RemoveNote) => {
                let transition = self
                    .glues
                    .dispatch(NotebookEvent::RemoveNote.into())
                    .log_unwrap();
                self.handle_transition(transition);
            }
            Action::Tui(TuiAction::AddNote) => {
                let note_name = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if note_name.is_empty() {
                    self.context.alert = Some("Note name cannot be empty".to_string());
                    return;
                }

                let transition = self
                    .glues
                    .dispatch(NotebookEvent::AddNote(note_name).into())
                    .log_unwrap();
                self.handle_transition(transition);
            }
            Action::Tui(TuiAction::AddDirectory) => {
                let directory_name = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if directory_name.is_empty() {
                    self.context.alert = Some("Directory name cannot be empty".to_string());
                    return;
                }

                let transition = self
                    .glues
                    .dispatch(NotebookEvent::AddDirectory(directory_name).into())
                    .log_unwrap();
                self.handle_transition(transition);
            }
            Action::Tui(TuiAction::RenameDirectory) => {
                let new_name = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if new_name.is_empty() {
                    self.context.alert = Some("Directory name cannot be empty".to_string());
                    return;
                }

                let transition = self
                    .glues
                    .dispatch(NotebookEvent::RenameDirectory(new_name).into())
                    .log_unwrap();
                self.handle_transition(transition);
            }
            Action::Tui(TuiAction::RemoveDirectory) => {
                let transition = self
                    .glues
                    .dispatch(NotebookEvent::RemoveDirectory.into())
                    .log_unwrap();
                self.handle_transition(transition);
            }
            Action::Dispatch(event) => {
                let transition = self.glues.dispatch(event).log_unwrap();
                self.handle_transition(transition);
            }
            Action::PassThrough => {
                let event = match to_event(input) {
                    Some(event) => event.into(),
                    None => {
                        return;
                    }
                };

                let transition = self.glues.dispatch(event).log_unwrap();
                self.handle_transition(transition);
            }
            Action::None => {}
        };
    }
}

fn to_event(input: Input) -> Option<KeyEvent> {
    let code = match input {
        Input::Key(key) => key.code,
        _ => return None,
    };

    let event = match code {
        KeyCode::Char('b') => KeyEvent::B,
        KeyCode::Char('e') => KeyEvent::E,
        KeyCode::Char('h') => KeyEvent::H,
        KeyCode::Char('j') => KeyEvent::J,
        KeyCode::Char('k') => KeyEvent::K,
        KeyCode::Char('l') => KeyEvent::L,
        KeyCode::Char('m') => KeyEvent::M,
        KeyCode::Char('o') => KeyEvent::O,
        KeyCode::Left => KeyEvent::Left,
        KeyCode::Right => KeyEvent::Right,
        KeyCode::Esc => KeyEvent::Esc,
        _ => return None,
    };

    Some(event)
}
