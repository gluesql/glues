use {
    super::{
        config::{
            self, LAST_CSV_PATH, LAST_FILE_PATH, LAST_GIT_BRANCH, LAST_GIT_PATH, LAST_GIT_REMOTE,
            LAST_JSON_PATH,
        },
        context::ContextPrompt,
        logger::*,
        App,
    },
    glues_core::{EntryEvent, Event, KeyEvent, NotebookEvent},
    ratatui::{
        crossterm::event::{Event as Input, KeyCode},
        style::Stylize,
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
    Alert(String),
    Confirm {
        message: String,
        action: Box<Action>,
    },
    Prompt {
        message: Vec<Line<'static>>,
        action: Box<Action>,
        default: Option<String>,
    },
    Help,
    ShowEditorKeymap,
    Quit,

    OpenCsv,
    OpenJson,
    OpenFile,
    OpenGit(OpenGitStep),

    RenameNote,
    RemoveNote,
    AddNote,
    AddDirectory,
    RenameDirectory,
    RemoveDirectory,
}

#[derive(Clone)]
pub enum OpenGitStep {
    Path,
    Remote { path: String },
    Branch { path: String, remote: String },
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
            Action::Tui(TuiAction::Help) => {
                self.context.help = true;
            }
            Action::Tui(TuiAction::ShowEditorKeymap) => {
                self.context.editor_keymap = true;
            }
            Action::Tui(TuiAction::Alert(message)) => {
                self.context.alert = Some(message);
            }
            Action::Tui(TuiAction::Confirm { message, action }) => {
                self.context.confirm = Some((message, *action));
            }
            Action::Tui(TuiAction::Prompt {
                message,
                action,
                default,
            }) => {
                self.context.prompt = Some(ContextPrompt::new(message, *action, default));
            }
            Action::Tui(TuiAction::OpenGit(OpenGitStep::Path)) => {
                let path = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                let message = vec![
                    Line::from(format!("path: {path}").dark_gray()),
                    Line::raw(""),
                    Line::raw("Enter the git remote:"),
                ];

                config::update(LAST_GIT_PATH, &path);
                let default = config::get(LAST_GIT_REMOTE);
                let action = TuiAction::OpenGit(OpenGitStep::Remote { path }).into();
                self.context.prompt = Some(ContextPrompt::new(message, action, default));
            }
            Action::Tui(TuiAction::OpenGit(OpenGitStep::Remote { path })) => {
                let remote = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                let message = vec![
                    Line::from(format!("path: {path}").dark_gray()),
                    Line::from(format!("remote: {remote}").dark_gray()),
                    Line::raw(""),
                    Line::raw("Enter the git branch:"),
                ];

                config::update(LAST_GIT_REMOTE, &remote);
                let default = config::get(LAST_GIT_BRANCH);
                let action = TuiAction::OpenGit(OpenGitStep::Branch { path, remote }).into();
                self.context.prompt = Some(ContextPrompt::new(message, action, default));
            }
            Action::Tui(TuiAction::OpenGit(OpenGitStep::Branch { path, remote })) => {
                let branch = self
                    .context
                    .take_prompt_input()
                    .log_expect("branch must not be none");
                let transition = self
                    .glues
                    .dispatch(
                        EntryEvent::OpenGit {
                            path,
                            remote,
                            branch,
                        }
                        .into(),
                    )
                    .log_unwrap();
                self.handle_transition(transition);
            }
            Action::Tui(TuiAction::OpenCsv) => {
                let path = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if path.is_empty() {
                    self.context.alert = Some("Path cannot be empty".to_string());
                    return;
                }

                config::update(LAST_CSV_PATH, &path);

                let transition = self
                    .glues
                    .dispatch(EntryEvent::OpenCsv(path).into())
                    .log_unwrap();
                self.handle_transition(transition);
            }
            Action::Tui(TuiAction::OpenJson) => {
                let path = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if path.is_empty() {
                    self.context.alert = Some("Path cannot be empty".to_string());
                    return;
                }

                config::update(LAST_JSON_PATH, &path);

                let transition = self
                    .glues
                    .dispatch(EntryEvent::OpenJson(path).into())
                    .log_unwrap();
                self.handle_transition(transition);
            }
            Action::Tui(TuiAction::OpenFile) => {
                let path = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if path.is_empty() {
                    self.context.alert = Some("Path cannot be empty".to_string());
                    return;
                }

                config::update(LAST_FILE_PATH, &path);

                let transition = self
                    .glues
                    .dispatch(EntryEvent::OpenFile(path).into())
                    .log_unwrap();
                self.handle_transition(transition);
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
