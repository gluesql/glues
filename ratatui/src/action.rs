use {
    super::{logger::*, App},
    glues_core::{EntryEvent, Event, KeyEvent, NotebookEvent},
    ratatui::crossterm::event::{KeyCode, KeyEvent as CKeyEvent},
};

pub enum Action {
    Tui(TuiAction),
    Dispatch(Event),
    PassThrough,
    None,
}

pub enum TuiAction {
    Confirm {
        message: String,
        action: Box<Action>,
    },
    Quit,

    RemoveNote,
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
    pub(super) fn handle_action(&mut self, action: Action, key: CKeyEvent) {
        match action {
            Action::Tui(TuiAction::Quit) => {}
            Action::Tui(TuiAction::Confirm { message, action }) => {
                self.context.confirm = Some((message, *action));
            }
            Action::Tui(TuiAction::RemoveNote) => {
                let transition = self
                    .glues
                    .dispatch(NotebookEvent::RemoveNote.into())
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
                let event = match to_event(key.code) {
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

fn to_event(code: KeyCode) -> Option<KeyEvent> {
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
