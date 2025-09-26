use {
    crate::{context::Context, views},
    glues_core::Glues,
    ratatui::Frame,
};

#[cfg(not(target_arch = "wasm32"))]
use {
    crate::{
        input::{Input, KeyCode, KeyEvent, KeyEventKind},
        logger::*,
    },
    ratatui::DefaultTerminal,
    std::time::Duration,
};

#[cfg(target_arch = "wasm32")]
#[cfg(not(target_arch = "wasm32"))]
use crate::logger::*;

pub struct App {
    pub(crate) glues: Glues,
    pub(crate) context: Context,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let glues = Glues::new();
        let context = Context::default();

        Self { glues, context }
    }

    #[doc(hidden)]
    pub fn glues_mut(&mut self) -> &mut Glues {
        // Test-only escape hatch. Use this to simulate external backend/state
        // mutations (e.g. another session creates directories) that cannot be
        // reproduced through the TUI input pipeline. Production code must go
        // through the normal event/transition flow.
        &mut self.glues
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        use ratatui::crossterm as ct;

        loop {
            if self
                .context
                .last_log
                .as_ref()
                .is_some_and(|(_, created_at)| created_at.elapsed().log_unwrap().as_secs() > 5)
            {
                self.context.last_log = None;
            }

            terminal.draw(|frame| self.draw(frame))?;

            if !ct::event::poll(Duration::from_millis(1500))? {
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

            let raw_input = ct::event::read()?;
            let input: Input = raw_input.into();

            if !matches!(
                input,
                Input::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                })
            ) {
                continue;
            }

            match input {
                Input::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers,
                    ..
                }) if modifiers.ctrl => {
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

    pub fn draw(&mut self, frame: &mut Frame) {
        use ratatui::layout::{
            Constraint::{Length, Percentage},
            Layout,
        };

        let state = &self.glues.state;
        let context = &mut self.context;
        let vertical = Layout::vertical([Length(1), Percentage(100)]);
        let [statusbar, body] = vertical.areas(frame.area());

        views::statusbar::draw(frame, statusbar, state, &context.notebook);
        views::body::draw(frame, body, context);
        views::dialog::draw(frame, state, context);
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}
