pub mod db;
mod error;
mod event;
mod glues;
mod schema;
mod task;

pub mod data;
pub mod state;
pub mod transition;
pub mod types;
pub mod proxy;

pub use error::Error;
pub use event::{EntryEvent, Event, KeyEvent, NotebookEvent, NumKey};
pub use glues::Glues;
pub use transition::{EntryTransition, NotebookTransition, Transition};

type Result<T> = std::result::Result<T, Error>;
