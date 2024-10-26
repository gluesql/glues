mod entry;
pub mod notebook;

use crate::{Error, Event, Glues, Result, Transition};

pub use {entry::EntryState, notebook::NotebookState};

pub enum State {
    EntryState(EntryState),
    NotebookState(NotebookState),
}

impl State {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<Transition> {
        match &glues.state {
            State::EntryState(_) => EntryState::consume(glues, event).await.map(Into::into),
            State::NotebookState(_) => notebook::consume(glues, event).await.map(Into::into),
        }
    }

    pub fn describe(&self) -> Result<String> {
        match self {
            Self::EntryState(state) => state.describe(),
            Self::NotebookState(state) => state.describe(),
        }
    }

    pub fn shortcuts(&self) -> Vec<String> {
        match self {
            Self::EntryState(state) => state.shortcuts(),
            Self::NotebookState(state) => state.shortcuts(),
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
                match self {
                    Self::$State(state) => Ok(&state),
                    _ => Err(Error::Wip("State::get_inner for $State failed".to_owned())),
                }
            }

            fn get_inner_mut(&mut self) -> Result<&mut $State> {
                match self {
                    Self::$State(state) => Ok(state),
                    _ => Err(Error::Wip(
                        "State::get_inner_mut for $State failed".to_owned(),
                    )),
                }
            }
        }

        impl From<$State> for State {
            fn from(state: $State) -> Self {
                Self::$State(state)
            }
        }
    };
}

impl_state_ext!(EntryState);
impl_state_ext!(NotebookState);
