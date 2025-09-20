pub mod backend;
mod error;
mod event;
mod glues;
mod schema;
#[cfg(not(target_arch = "wasm32"))]
mod task;

pub mod data;
pub mod state;
pub mod transition;
pub mod types;

pub use backend::CoreBackend;
pub use error::Error;
pub use event::{EntryEvent, Event, KeyEvent, NotebookEvent, NumKey};
pub use glues::Glues;
#[cfg(not(target_arch = "wasm32"))]
pub use task::{Task, handle_tasks};
pub use transition::{EntryTransition, NotebookTransition, Transition};

type Result<T> = std::result::Result<T, Error>;
