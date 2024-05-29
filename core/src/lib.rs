mod error;
mod glues;
mod schema;

pub mod data;
pub mod types;

pub use error::Error;
pub use glues::Glues;

type Result<T> = std::result::Result<T, Error>;
