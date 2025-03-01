mod entry;
pub mod notebook;

use crate::{Error, Event, Glues, KeyEvent, Result, Transition, transition::KeymapTransition};

pub use {entry::EntryState, notebook::NotebookState};

pub struct State {
    pub keymap: bool,
    inner: InnerState,
}

pub enum InnerState {
    EntryState(Box<EntryState>),
    NotebookState(Box<NotebookState>),
}

impl State {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<Transition> {
        match event {
            Event::Key(KeyEvent::QuestionMark) if glues.state.keymap => {
                glues.state.keymap = false;

                return Ok(KeymapTransition::Hide.into());
            }
            Event::Key(KeyEvent::QuestionMark) => {
                glues.state.keymap = true;

                return Ok(KeymapTransition::Show.into());
            }
            _ => {}
        };

        match &glues.state.inner {
            InnerState::EntryState(_) => EntryState::consume(glues, event).await.map(Into::into),
            InnerState::NotebookState(_) => notebook::consume(glues, event).await.map(Into::into),
        }
    }

    pub fn describe(&self) -> Result<String> {
        match &self.inner {
            InnerState::EntryState(state) => state.describe(),
            InnerState::NotebookState(state) => state.describe(),
        }
    }

    pub fn shortcuts(&self) -> Vec<String> {
        match &self.inner {
            InnerState::EntryState(state) => state.shortcuts(),
            InnerState::NotebookState(state) => state.shortcuts(),
        }
    }
}

pub trait GetInner<T> {
    fn get_inner(&self) -> Result<&T>;

    fn get_inner_mut(&mut self) -> Result<&mut T>;
}

macro_rules! impl_state_ext {
    ($State: ident) => {
        impl GetInner<$State> for State {
            fn get_inner(&self) -> Result<&$State> {
                match &self.inner {
                    InnerState::$State(state) => Ok(&state),
                    _ => Err(Error::Wip("State::get_inner for $State failed".to_owned())),
                }
            }

            fn get_inner_mut(&mut self) -> Result<&mut $State> {
                match &mut self.inner {
                    InnerState::$State(state) => Ok(state),
                    _ => Err(Error::Wip(
                        "State::get_inner_mut for $State failed".to_owned(),
                    )),
                }
            }
        }

        impl From<$State> for State {
            fn from(state: $State) -> Self {
                Self {
                    keymap: false,
                    inner: InnerState::$State(Box::new(state)),
                }
            }
        }
    };
}

impl_state_ext!(EntryState);
impl_state_ext!(NotebookState);
