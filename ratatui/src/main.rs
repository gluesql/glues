mod action;
pub mod context;
#[macro_use]
mod logger;
mod transitions;
mod views;

use {
    action::{Action, TuiAction},
    color_eyre::Result,
    context::Context,
    futures::executor::block_on,
    glues_core::Glues,
    logger::*,
    ratatui::{
        crossterm::{
            self,
            event::{Event as Input, KeyCode, KeyEvent as CKeyEvent, KeyEventKind, KeyModifiers},
        },
        layout::{
            Constraint::{Length, Percentage},
            Layout,
        },
        DefaultTerminal, Frame,
    },
    std::time::Duration,
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
        let context = Context::default();

        Self { glues, context }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if !crossterm::event::poll(Duration::from_millis(1500))? {
                let mut transitions = Vec::new();
                {
                    let mut queue = self.glues.transition_queue.lock().log_unwrap();

                    while let Some(transition) = queue.pop_front() {
                        transitions.push(transition);
                    }
                }

                for transition in transitions {
                    self.handle_transition(transition);
                }
            }

            let input = crossterm::event::read()?;
            if !matches!(
                input,
                Input::Key(CKeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                })
            ) {
                continue;
            }

            match input {
                Input::Key(CKeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => {
                    return Ok(());
                }
                _ => {
                    match self.context.consume(&input) {
                        Action::Tui(TuiAction::Quit) => return Ok(()),
                        action => {
                            self.handle_action(action, input);
                        }
                    };
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let state = &self.glues.state;
        let context = &mut self.context;
        let vertical = Layout::vertical([Length(1), Percentage(100)]);
        let [statusbar, body] = vertical.areas(frame.area());

        views::statusbar::draw(frame, statusbar, state);
        views::body::draw(frame, body, context);
        views::dialog::draw(frame, context);
    }
}
