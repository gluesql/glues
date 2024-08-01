mod entry;
pub mod notes;

use crate::{Error, Event, Glues, Result, Transition};

pub use {entry::EntryState, notes::NotesState};

pub enum State {
    EntryState(EntryState),
    NotesState(NotesState),
}

impl State {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<Transition> {
        match &glues.state {
            State::EntryState(_) => EntryState::consume(glues, event).await,
            State::NotesState(_) => notes::consume(glues, event).await,
        }
    }

    pub fn describe(&self) -> Result<String> {
        match self {
            Self::EntryState(state) => state.describe(),
            Self::NotesState(state) => state.describe(),
        }
    }

    pub fn shortcuts(&self) -> Vec<&str> {
        match self {
            Self::EntryState(state) => state.shortcuts(),
            Self::NotesState(state) => state.shortcuts(),
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
impl_state_ext!(NotesState);
