use {
    crate::{Result, schema::setup, types::DirectoryId},
    async_trait::async_trait,
    gluesql::{
        core::ast_builder::Build,
        prelude::{Glue, IdbStorage, MemoryStorage, Payload},
    },
};

pub enum Storage {
    Memory(Glue<MemoryStorage>),
    IndexedDb(Glue<IdbStorage>),
}

pub struct Db {
    pub storage: Storage,
    pub root_id: DirectoryId,
}

impl Db {
    pub async fn memory() -> Result<Self> {
        let mut storage = Storage::Memory(Glue::new(MemoryStorage::default()));
        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
    }

    pub async fn indexed_db(namespace: Option<String>) -> Result<Self> {
        let idb = IdbStorage::new(namespace).await?;
        let mut storage = Storage::IndexedDb(Glue::new(idb));
        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
    }

    pub fn sync(&self) -> Result<()> {
        Ok(())
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
impl<T> Execute for T
where
    T: Build + Send,
{
    async fn execute(self, storage: &mut Storage) -> Result<Payload> {
        let statement = self.build()?;

        match storage {
            Storage::Memory(glue) => glue.execute_stmt(&statement).await,
            Storage::IndexedDb(glue) => glue.execute_stmt(&statement).await,
        }
        .map_err(Into::into)
    }
}
