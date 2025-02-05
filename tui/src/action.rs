use {
    super::{
        color::*,
        config::{
            self, LAST_CSV_PATH, LAST_FILE_PATH, LAST_GIT_BRANCH, LAST_GIT_PATH, LAST_GIT_REMOTE,
            LAST_JSON_PATH, LAST_MONGO_CONN_STR, LAST_MONGO_DB_NAME,
        },
        context::ContextPrompt,
        logger::*,
        App,
    },
    glues_core::{EntryEvent, Event, KeyEvent, NotebookEvent, NumKey},
    ratatui::{
        crossterm::event::{Event as Input, KeyCode, KeyModifiers},
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
    SaveAndConfirm {
        // todo: there might be a better way to do this
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
    SaveAndPassThrough,
    Quit,

    OpenCsv,
    OpenJson,
    OpenFile,
    OpenGit(OpenGitStep),
    OpenMongo(OpenMongoStep),

    RenameNote,
    RemoveNote,
    AddNote,
    AddDirectory,
    RenameDirectory,
    RemoveDirectory,
}

#[derive(Clone)]
pub enum OpenMongoStep {
    ConnStr,
    Database { conn_str: String },
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
    pub(super) async fn handle_action(&mut self, action: Action, input: Input) -> bool {
        match action {
            Action::Tui(TuiAction::Quit) => {
                return true;
            }
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
            Action::Tui(TuiAction::SaveAndConfirm { message, action }) => {
                self.save().await;
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
                    Line::from(format!("path: {path}").fg(GRAY_MEDIUM)),
                    Line::raw(""),
                    Line::raw("Enter the git remote:"),
                ];

                config::update(LAST_GIT_PATH, &path).await;
                let default = config::get(LAST_GIT_REMOTE).await;
                let action = TuiAction::OpenGit(OpenGitStep::Remote { path }).into();
                self.context.prompt = Some(ContextPrompt::new(message, action, default));
            }
            Action::Tui(TuiAction::OpenGit(OpenGitStep::Remote { path })) => {
                let remote = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                let message = vec![
                    Line::from(format!("path: {path}").fg(GRAY_MEDIUM)),
                    Line::from(format!("remote: {remote}").fg(GRAY_MEDIUM)),
                    Line::raw(""),
                    Line::raw("Enter the git branch:"),
                ];

                config::update(LAST_GIT_REMOTE, &remote).await;
                let default = config::get(LAST_GIT_BRANCH).await;
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
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Tui(TuiAction::OpenMongo(OpenMongoStep::ConnStr)) => {
                let conn_str = self
                    .context
                    .take_prompt_input()
                    .log_expect("conn str must not be none");
                let message = vec![
                    Line::from(format!("conn_str: {conn_str}").fg(GRAY_MEDIUM)),
                    Line::raw(""),
                    Line::raw("Enter the database name:"),
                ];

                config::update(LAST_MONGO_CONN_STR, &conn_str).await;
                let default = config::get(LAST_MONGO_DB_NAME).await;

                let action = TuiAction::OpenMongo(OpenMongoStep::Database { conn_str }).into();
                self.context.prompt = Some(ContextPrompt::new(message, action, default));
            }
            Action::Tui(TuiAction::OpenMongo(OpenMongoStep::Database { conn_str })) => {
                let db_name = self
                    .context
                    .take_prompt_input()
                    .log_expect("database name must not be none");

                config::update(LAST_MONGO_DB_NAME, &db_name).await;

                let transition = self
                    .glues
                    .dispatch(EntryEvent::OpenMongo { conn_str, db_name }.into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Tui(TuiAction::OpenCsv) => {
                let path = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if path.is_empty() {
                    self.context.alert = Some("Path cannot be empty".to_string());
                    return false;
                }

                config::update(LAST_CSV_PATH, &path).await;

                let transition = self
                    .glues
                    .dispatch(EntryEvent::OpenCsv(path).into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Tui(TuiAction::OpenJson) => {
                let path = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if path.is_empty() {
                    self.context.alert = Some("Path cannot be empty".to_string());
                    return false;
                }

                config::update(LAST_JSON_PATH, &path).await;

                let transition = self
                    .glues
                    .dispatch(EntryEvent::OpenJson(path).into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Tui(TuiAction::OpenFile) => {
                let path = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if path.is_empty() {
                    self.context.alert = Some("Path cannot be empty".to_string());
                    return false;
                }

                config::update(LAST_FILE_PATH, &path).await;

                let transition = self
                    .glues
                    .dispatch(EntryEvent::OpenFile(path).into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Tui(TuiAction::RenameNote) => {
                let new_name = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if new_name.is_empty() {
                    self.context.alert = Some("Note name cannot be empty".to_string());
                    return false;
                }

                let transition = self
                    .glues
                    .dispatch(NotebookEvent::RenameNote(new_name).into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Tui(TuiAction::RemoveNote) => {
                let transition = self
                    .glues
                    .dispatch(NotebookEvent::RemoveNote.into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Tui(TuiAction::AddNote) => {
                let note_name = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if note_name.is_empty() {
                    self.context.alert = Some("Note name cannot be empty".to_string());
                    return false;
                }

                let transition = self
                    .glues
                    .dispatch(NotebookEvent::AddNote(note_name).into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Tui(TuiAction::AddDirectory) => {
                let directory_name = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if directory_name.is_empty() {
                    self.context.alert = Some("Directory name cannot be empty".to_string());
                    return false;
                }

                let transition = self
                    .glues
                    .dispatch(NotebookEvent::AddDirectory(directory_name).into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Tui(TuiAction::RenameDirectory) => {
                let new_name = self
                    .context
                    .take_prompt_input()
                    .log_expect("prompt must not be none");
                if new_name.is_empty() {
                    self.context.alert = Some("Directory name cannot be empty".to_string());
                    return false;
                }

                let transition = self
                    .glues
                    .dispatch(NotebookEvent::RenameDirectory(new_name).into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Tui(TuiAction::RemoveDirectory) => {
                let transition = self
                    .glues
                    .dispatch(NotebookEvent::RemoveDirectory.into())
                    .await
                    .log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::Dispatch(event) => {
                let transition = self.glues.dispatch(event).await.log_unwrap();
                self.handle_transition(transition).await;
            }

            Action::Tui(TuiAction::SaveAndPassThrough) => {
                self.save().await;

                let event = match to_event(input) {
                    Some(event) => event.into(),
                    None => {
                        return false;
                    }
                };

                let transition = self.glues.dispatch(event).await.log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::PassThrough => {
                let event = match to_event(input) {
                    Some(event) => event.into(),
                    None => {
                        return false;
                    }
                };

                let transition = self.glues.dispatch(event).await.log_unwrap();
                self.handle_transition(transition).await;
            }
            Action::None => {}
        };

        false
    }
}

fn to_event(input: Input) -> Option<KeyEvent> {
    let key = match input {
        Input::Key(key) => key,
        _ => return None,
    };
    let code = key.code;
    let ctrl = key.modifiers == KeyModifiers::CONTROL;

    let event = match code {
        KeyCode::Char('h') if ctrl => KeyEvent::CtrlH,
        KeyCode::Char('r') if ctrl => KeyEvent::CtrlR,
        KeyCode::Char('a') => KeyEvent::A,
        KeyCode::Char('b') => KeyEvent::B,
        KeyCode::Char('c') => KeyEvent::C,
        KeyCode::Char('d') => KeyEvent::D,
        KeyCode::Char('e') => KeyEvent::E,
        KeyCode::Char('g') => KeyEvent::G,
        KeyCode::Char('h') => KeyEvent::H,
        KeyCode::Char('i') => KeyEvent::I,
        KeyCode::Char('j') => KeyEvent::J,
        KeyCode::Char('k') => KeyEvent::K,
        KeyCode::Char('l') => KeyEvent::L,
        KeyCode::Char('m') => KeyEvent::M,
        KeyCode::Char('n') => KeyEvent::N,
        KeyCode::Char('o') => KeyEvent::O,
        KeyCode::Char('p') => KeyEvent::P,
        KeyCode::Char('s') => KeyEvent::S,
        KeyCode::Char('t') => KeyEvent::T,
        KeyCode::Char('u') => KeyEvent::U,
        KeyCode::Char('v') => KeyEvent::V,
        KeyCode::Char('w') => KeyEvent::W,
        KeyCode::Char('x') => KeyEvent::X,
        KeyCode::Char('y') => KeyEvent::Y,
        KeyCode::Char('z') => KeyEvent::Z,
        KeyCode::Char('A') => KeyEvent::CapA,
        KeyCode::Char('G') => KeyEvent::CapG,
        KeyCode::Char('H') => KeyEvent::CapH,
        KeyCode::Char('I') => KeyEvent::CapI,
        KeyCode::Char('L') => KeyEvent::CapL,
        KeyCode::Char('O') => KeyEvent::CapO,
        KeyCode::Char('S') => KeyEvent::CapS,
        KeyCode::Char('U') => KeyEvent::CapU,
        KeyCode::Char('X') => KeyEvent::CapX,
        KeyCode::Char('1') => NumKey::One.into(),
        KeyCode::Char('2') => NumKey::Two.into(),
        KeyCode::Char('3') => NumKey::Three.into(),
        KeyCode::Char('4') => NumKey::Four.into(),
        KeyCode::Char('5') => NumKey::Five.into(),
        KeyCode::Char('6') => NumKey::Six.into(),
        KeyCode::Char('7') => NumKey::Seven.into(),
        KeyCode::Char('8') => NumKey::Eight.into(),
        KeyCode::Char('9') => NumKey::Nine.into(),
        KeyCode::Char('0') => NumKey::Zero.into(),
        KeyCode::Char('$') => KeyEvent::DollarSign,
        KeyCode::Char('^') => KeyEvent::Caret,
        KeyCode::Char('~') => KeyEvent::Tilde,
        KeyCode::Char('?') => KeyEvent::QuestionMark,
        KeyCode::Char('.') => KeyEvent::Dot,
        KeyCode::Char('-') => KeyEvent::Dash,
        KeyCode::Char(' ') => KeyEvent::Space,
        KeyCode::Left => KeyEvent::Left,
        KeyCode::Right => KeyEvent::Right,
        KeyCode::Up => KeyEvent::Up,
        KeyCode::Down => KeyEvent::Down,
        KeyCode::Enter => KeyEvent::Enter,
        KeyCode::Tab => KeyEvent::Tab,
        KeyCode::Esc => KeyEvent::Esc,
        _ => return None,
    };

    Some(event)
}
