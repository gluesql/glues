use {
    crate::{action::Action, config, context::Context, logger::*, theme, transitions, views},
    color_eyre::Result,
    glues_core::Glues,
    ratatui::crossterm::event::{
        Event as Input, KeyCode, KeyEvent as CKeyEvent, KeyEventKind, KeyModifiers,
    },
    ratatui::{
        Frame,
        layout::{
            Constraint::{Length, Percentage},
            Layout,
        },
        prelude::Backend,
    },
    ratzilla::{
        DomBackend, WebRenderer,
        event::{KeyCode as RzKeyCode, KeyEvent as RzKeyEvent},
    },
    std::{
        cell::RefCell,
        collections::VecDeque,
        rc::Rc,
        time::{Duration, Instant},
    },
};

fn rz_to_crossterm(ev: RzKeyEvent) -> Input {
    let code = match ev.code {
        RzKeyCode::Char(c) => KeyCode::Char(c),
        RzKeyCode::F(n) => KeyCode::F(n),
        RzKeyCode::Backspace => KeyCode::Backspace,
        RzKeyCode::Enter => KeyCode::Enter,
        RzKeyCode::Left => KeyCode::Left,
        RzKeyCode::Right => KeyCode::Right,
        RzKeyCode::Up => KeyCode::Up,
        RzKeyCode::Down => KeyCode::Down,
        RzKeyCode::Tab => KeyCode::Tab,
        RzKeyCode::Delete => KeyCode::Delete,
        RzKeyCode::Home => KeyCode::Home,
        RzKeyCode::End => KeyCode::End,
        RzKeyCode::PageUp => KeyCode::PageUp,
        RzKeyCode::PageDown => KeyCode::PageDown,
        RzKeyCode::Esc => KeyCode::Esc,
        RzKeyCode::Unidentified => KeyCode::Null,
    };

    let mut modifiers = KeyModifiers::empty();
    if ev.ctrl {
        modifiers |= KeyModifiers::CONTROL;
    }
    if ev.alt {
        modifiers |= KeyModifiers::ALT;
    }
    if ev.shift {
        modifiers |= KeyModifiers::SHIFT;
    }

    Input::Key(CKeyEvent::new(code, modifiers))
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

    async fn run<B: Backend + 'static>(
        mut self,
        mut terminal: ratatui::Terminal<B>,
        event_queue: Rc<RefCell<VecDeque<Input>>>,
    ) -> Result<()> {
        loop {
            if let Some((_, created_at)) = self.context.last_log {
                if created_at.elapsed().as_secs() > 5 {
                    self.context.last_log = None;
                }
            }

            terminal.draw(|f| self.draw(f))?;

            let start = Instant::now();
            while start.elapsed() < Duration::from_millis(1500) {
                if let Some(input) = event_queue.borrow_mut().pop_front() {
                    match input {
                        Input::Key(CKeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers,
                            ..
                        }) if modifiers.contains(KeyModifiers::CONTROL) => {
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
                } else {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            }

            let mut transitions = Vec::new();
            {
                let mut queue = self.glues.transition_queue.lock().unwrap();
                while let Some(t) = queue.pop_front() {
                    transitions.push(t);
                }
            }
            for t in transitions {
                self.handle_transition(t).await;
            }
            self.save().await;
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let state = &self.glues.state;
        let context = &mut self.context;
        let vertical = Layout::vertical([Length(1), Percentage(100)]);
        let [statusbar, body] = vertical.areas(frame.area());
        views::statusbar::draw(frame, statusbar, state, &context.notebook);
        views::body::draw(frame, body, context);
        views::dialog::draw(frame, state, context);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    theme::set_theme(theme::DARK_THEME);
    config::init().await;
    logger::init().await;
    color_eyre::install()?;

    log!("Hello");

    let backend = DomBackend::new()?;
    let mut terminal = ratatui::Terminal::new(backend)?;
    let event_queue = Rc::new(RefCell::new(VecDeque::new()));
    {
        let queue = event_queue.clone();
        terminal.on_key_event(move |e| {
            queue.borrow_mut().push_back(rz_to_crossterm(e));
        });
    }

    App::new().await.run(terminal, event_queue).await
}
