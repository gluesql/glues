mod directory;
mod note;

use {
    crate::Result,
    async_trait::async_trait,
    gluesql::{
        core::ast_builder::Build,
        prelude::{CsvStorage, Glue, JsonStorage, MemoryStorage, Payload},
    },
};

pub struct Db {
    pub storage: Storage,
}

pub enum Storage {
    Memory(Glue<MemoryStorage>),
    Csv(Glue<CsvStorage>),
    Json(Glue<JsonStorage>),
}

impl Db {
    pub fn memory() -> Self {
        let storage = MemoryStorage::default();
        let glue = Glue::new(storage);

        Self {
            storage: Storage::Memory(glue),
        }
    }

    pub fn csv(path: &str) -> Result<Self> {
        CsvStorage::new(path)
            .map_err(Into::into)
            .map(Glue::new)
            .map(Storage::Csv)
            .map(|storage| Self { storage })
    }

    pub fn json(path: &str) -> Result<Self> {
        JsonStorage::new(path)
            .map_err(Into::into)
            .map(Glue::new)
            .map(Storage::Json)
            .map(|storage| Self { storage })
    }
}

#[async_trait(?Send)]
pub trait Execute
where
    Self: Sized,
{
    async fn execute(self, storage: &mut Storage) -> Result<Payload>;
}

#[async_trait(?Send)]
impl<T: Build> Execute for T
where
    Self: Sized,
{
    async fn execute(self, storage: &mut Storage) -> Result<Payload> {
        let statement = self.build()?;

        match storage {
            Storage::Memory(glue) => glue.execute_stmt(&statement).await,
            Storage::Csv(glue) => glue.execute_stmt(&statement).await,
            Storage::Json(glue) => glue.execute_stmt(&statement).await,
        }
        .map_err(Into::into)
    }
}
