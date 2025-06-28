use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("gluesql: {0}")]
    GlueSql(#[from] gluesql::prelude::Error),

    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("invalid response: {0}")]
    InvalidResponse(String),

    #[error("invalid state: {0}")]
    InvalidState(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("todo: {0}")]
    Todo(String),

    #[error("proxy: {0}")]
    Proxy(String),

    #[error("backend: {0}")]
    BackendError(String),
}
