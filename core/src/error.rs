use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("gluesql: {0}")]
    GlueSql(#[from] gluesql::prelude::Error),

    #[error("wip: {0}")]
    Wip(String),
}
