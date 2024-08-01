mod db;
mod error;
mod event;
mod glues;
mod schema;

pub mod data;
pub mod state;
pub mod transition;
pub mod types;

pub use error::Error;
pub use event::{EntryEvent, Event, KeyEvent, NotebookEvent};
pub use glues::Glues;
pub use transition::{EntryTransition, NotebookTransition, Transition};

type Result<T> = std::result::Result<T, Error>;
