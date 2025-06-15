mod action;
pub mod context;
#[macro_use]
pub mod logger;
mod color;
pub mod config;
pub mod theme;
mod transitions;
mod views;

include!("app.rs");
