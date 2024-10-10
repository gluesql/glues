mod action;
mod context;
#[macro_use]
mod logger;
mod transitions;
mod views;

use {
    action::{Action, TuiAction},
    color_eyre::Result,
    context::Context,
    futures::executor::block_on,
    glues_core::{state::State, Glues, KeyEvent},
    logger::*,
    ratatui::{
        crossterm::event::{
            self, Event, KeyCode, KeyEvent as CKeyEvent, KeyEventKind, KeyModifiers,
        },
        layout::{
            Constraint::{Length, Percentage},
            Layout,
        },
        DefaultTerminal, Frame,
    },
};

fn main() -> Result<()> {
    logger::init();
    color_eyre::install()?;

    log!("Hello");

    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    glues: Glues,
    context: Context,
}

impl App {
    fn new() -> Self {
        let glues = block_on(Glues::new());
        let context = Context::new();

        Self { glues, context }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if matches!(
                    key,
                    CKeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }
                ) {
                    return Ok(());
                }

                let action = match &self.glues.state {
                    State::EntryState(_) => self.context.entry.consume(key.code),
                    State::NotebookState(_) => self.context.notebook.consume(key.code),
                };

                match action {
                    Action::Tui(TuiAction::Quit) => return Ok(()),
                    Action::Dispatch(event) => {
                        let transition = self.glues.dispatch(event).log_unwrap();
                        self.handle_transition(transition);
                    }
                    Action::PassThrough => {
                        let event = match to_event(key.code) {
                            Some(event) => event.into(),
                            None => continue,
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
    }

    fn draw(&mut self, frame: &mut Frame) {
        let state = &self.glues.state;
        let context = &mut self.context;
        let vertical = Layout::vertical([Length(1), Percentage(100)]);
        let [statusbar, body] = vertical.areas(frame.area());

        views::statusbar::draw(frame, statusbar, state);
        views::body::draw(frame, body, state, context);
    }
}
