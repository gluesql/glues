mod action;
pub mod context;
#[macro_use]
mod logger;
mod color;
mod config;
mod theme;
mod transitions;
mod views;

use {
    action::Action,
    color_eyre::Result,
    context::Context,
    glues_core::Glues,
    logger::*,
    ratatui::{
        DefaultTerminal, Frame,
        crossterm::{
            self,
            event::{Event as Input, KeyCode, KeyEvent as CKeyEvent, KeyEventKind, KeyModifiers},
        },
        layout::{
            Constraint::{Length, Percentage},
            Layout,
        },
    },
    std::time::Duration,
};

#[tokio::main]
async fn main() -> Result<()> {
    config::init().await;
    logger::init().await;
    color_eyre::install()?;

    log!("Hello");

    let terminal = ratatui::init();
    let app_result = App::new().await.run(terminal).await;
    ratatui::restore();
    app_result
}

struct App {
    glues: Glues,
    context: Context,
}

impl App {
    async fn new() -> Self {
        let glues = Glues::new().await;
        let context = Context::default();

        Self { glues, context }
    }

    async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            if let Some((_, created_at)) = self.context.last_log {
                if created_at.elapsed().log_unwrap().as_secs() > 5 {
                    self.context.last_log = None;
                }
            }

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
                    self.handle_transition(transition).await;
                }

                self.save().await;
                continue;
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
                    self.save().await;
                    return Ok(());
                }
                _ => {
                    let action = self.context.consume(&input).await;
                    let quit = self.handle_action(action, input).await;
                    if quit {
                        return Ok(());
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let state = &self.glues.state;
        let context = &mut self.context;
        let vertical = Layout::vertical([Length(1), Percentage(100)]);
        let [statusbar, body] = vertical.areas(frame.area());

        if let Some(menu) = context.menu.as_mut() {
            views::menu::draw(frame, statusbar, menu);
        } else {
            views::statusbar::draw(frame, statusbar, state, &context.notebook);
        }
        views::body::draw(frame, body, context);
        views::dialog::draw(frame, state, context);
    }
}
