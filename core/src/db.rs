mod directory;
mod note;

use {
    crate::{schema::setup, types::DirectoryId, Result},
    async_trait::async_trait,
    gluesql::{
        core::ast_builder::Build,
        prelude::{CsvStorage, FileStorage, Glue, JsonStorage, MemoryStorage, Payload},
    },
};

pub struct Db {
    pub storage: Storage,
    pub root_id: DirectoryId,
}

pub enum Storage {
    Memory(Glue<MemoryStorage>),
    Csv(Glue<CsvStorage>),
    Json(Glue<JsonStorage>),
    File(Glue<FileStorage>),
}

impl Db {
    pub async fn memory() -> Result<Self> {
        let glue = Glue::new(MemoryStorage::default());
        let mut storage = Storage::Memory(glue);

        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
    }

    pub async fn csv(path: &str) -> Result<Self> {
        let mut storage = CsvStorage::new(path).map(Glue::new).map(Storage::Csv)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
    }

    pub async fn json(path: &str) -> Result<Self> {
        let mut storage = JsonStorage::new(path).map(Glue::new).map(Storage::Json)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
    }

    pub async fn file(path: &str) -> Result<Self> {
        let mut storage = FileStorage::new(path).map(Glue::new).map(Storage::File)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
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
            Storage::File(glue) => glue.execute_stmt(&statement).await,
        }
        .map_err(Into::into)
    }
}
