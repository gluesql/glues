use {
    crate::{context::Context, logger::*, views},
    glues_core::Glues,
    ratatui::{DefaultTerminal, Frame},
    std::time::Duration,
};

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

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        use ct::event::{
            Event as Input, KeyCode, KeyEvent as CKeyEvent, KeyEventKind, KeyModifiers,
        };
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

            let input = ct::event::read()?;
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

    // Test helper: render one frame on any ratatui Terminal
    #[cfg(feature = "test-utils")]
    pub fn draw_once_on<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut ratatui::Terminal<B>,
    ) -> color_eyre::Result<()> {
        terminal.draw(|f| self.draw(f))?;
        Ok(())
    }

    // Test helper: inject a single input event and return whether it requested quit
    #[cfg(feature = "test-utils")]
    pub async fn handle_input(&mut self, input: ratatui::crossterm::event::Event) -> bool {
        use ratatui::crossterm::event::{
            Event as Input, KeyCode, KeyEvent as CKeyEvent, KeyEventKind, KeyModifiers,
        };

        if !matches!(
            input,
            Input::Key(CKeyEvent {
                kind: KeyEventKind::Press,
                ..
            })
        ) {
            return false;
        }

        match input {
            Input::Key(CKeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                self.save_immediate().await;
                true
            }
            _ => {
                let action = self.context.consume(&input).await;
                self.handle_action(action, input).await
            }
        }
    }

    // Test helper: drain queued transitions (e.g., from background tasks)
    #[cfg(feature = "test-utils")]
    pub async fn drain_pending_transitions(&mut self) {
        let mut transitions = Vec::new();
        {
            let mut queue = self.glues.transition_queue.lock().log_unwrap();
            while let Some(t) = queue.pop_front() {
                transitions.push(t);
            }
        }
        for t in transitions {
            self.handle_transition(t).await;
        }
    }

    #[cfg(feature = "test-utils")]
    async fn save_immediate(&mut self) {
        self.save().await;
    }
}
