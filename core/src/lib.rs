mod error;
mod event;
mod glues;
mod schema;
mod transition;

pub mod data;
pub mod state;
pub mod types;

pub use error::Error;
pub use event::Event;
pub use glues::Glues;
pub use transition::Transition;

type Result<T> = std::result::Result<T, Error>;
