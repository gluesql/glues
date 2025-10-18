use {
    crate::{context::Context, views},
    glues_core::{Glues, transition::Transition},
    ratatui::Frame,
    std::{
        collections::VecDeque,
        sync::{
            Arc, Mutex,
            atomic::{AtomicBool, Ordering},
        },
    },
};

#[cfg(not(target_arch = "wasm32"))]
use {
    crate::{
        input::{Input, KeyCode, KeyEvent, KeyEventKind},
        logger::*,
    },
    ratatui::DefaultTerminal,
    std::time::Duration,
    tokio::{self, task},
};

#[cfg(target_arch = "wasm32")]
use crate::logger::*;

pub struct App {
    pub(crate) glues: Glues,
    pub(crate) context: Context,
    bg_transitions: Arc<Mutex<VecDeque<Transition>>>,
    sync_in_progress: Arc<AtomicBool>,
    #[cfg(not(target_arch = "wasm32"))]
    sync_pending: Arc<AtomicBool>,
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
        let bg_transitions = Arc::new(Mutex::new(VecDeque::new()));
        let sync_in_progress = Arc::new(AtomicBool::new(false));
        #[cfg(not(target_arch = "wasm32"))]
        let sync_pending = Arc::new(AtomicBool::new(false));

        Self {
            glues,
            context,
            bg_transitions,
            sync_in_progress,
            #[cfg(not(target_arch = "wasm32"))]
            sync_pending,
        }
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

            self.process_background().await;
            terminal.draw(|frame| self.draw(frame))?;

            if !ct::event::poll(Duration::from_millis(1500))? {
                self.process_background().await;
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

            self.process_background().await;
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

    async fn process_background(&mut self) {
        let mut transitions = Vec::new();

        {
            let mut queue = self.bg_transitions.lock().log_unwrap();
            while let Some(transition) = queue.pop_front() {
                transitions.push(transition);
            }
        }

        for transition in transitions {
            self.handle_transition(transition).await;
        }

        #[cfg(not(target_arch = "wasm32"))]
        self.flush_pending_sync();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn maybe_schedule_sync(&mut self) {
        if self.sync_in_progress.load(Ordering::SeqCst) {
            self.sync_pending.store(true, Ordering::SeqCst);
            return;
        }

        let Some(backend) = self.glues.db.as_mut() else {
            return;
        };
        let Some(job) = backend.sync_job() else {
            return;
        };

        self.sync_in_progress.store(true, Ordering::SeqCst);
        self.sync_pending.store(false, Ordering::SeqCst);
        let queue = Arc::clone(&self.bg_transitions);
        let flag = Arc::clone(&self.sync_in_progress);
        let pending = Arc::clone(&self.sync_pending);

        tokio::spawn(async move {
            let result = task::spawn_blocking(move || job.run()).await;
            let transition = match result {
                Ok(Ok(())) => {
                    Transition::Log("Sync complete. Your notes are up to date.".to_owned())
                }
                Ok(Err(err)) => Transition::Error(err.to_string()),
                Err(join_err) => Transition::Error(format!("Sync task panicked: {join_err}")),
            };

            {
                let mut guard = queue.lock().log_unwrap();
                guard.push_back(transition);
            }

            flag.store(false, Ordering::SeqCst);
            if pending.swap(false, Ordering::SeqCst) {
                pending.store(true, Ordering::SeqCst);
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn flush_pending_sync(&mut self) {
        if self.sync_in_progress.load(Ordering::SeqCst) {
            return;
        }

        if !self.sync_pending.swap(false, Ordering::SeqCst) {
            return;
        }

        self.maybe_schedule_sync();
    }
}
