use {
    crate::{Result, schema::setup, types::DirectoryId},
    async_trait::async_trait,
    gluesql::{
        core::ast_builder::Build,
        prelude::{Glue, MemoryStorage, Payload},
    },
};

pub type Storage = Glue<MemoryStorage>;

pub struct Db {
    pub storage: Storage,
    pub root_id: DirectoryId,
}

impl Db {
    pub async fn memory() -> Result<Self> {
        let mut storage = Glue::new(MemoryStorage::default());
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
        storage.execute_stmt(&statement).await.map_err(Into::into)
    }
}
