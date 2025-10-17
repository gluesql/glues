use {
    crate::{Error, Result, backend::SyncJob, schema::setup, types::DirectoryId},
    async_trait::async_trait,
    gluesql::{
        core::ast_builder::Build,
        gluesql_git_storage::{GitStorage, StorageType},
        gluesql_mongo_storage::MongoStorage,
        gluesql_redb_storage::RedbStorage,
        prelude::{FileStorage, Glue, MemoryStorage, Payload},
    },
};

pub struct Db {
    pub storage: Storage,
    pub root_id: DirectoryId,
}

pub enum Storage {
    Memory(Glue<MemoryStorage>),
    File(Glue<FileStorage>),
    Redb(Glue<RedbStorage>),
    Git(Glue<GitStorage>),
    Mongo(Glue<MongoStorage>),
}

impl Db {
    pub async fn memory() -> Result<Self> {
        let glue = Glue::new(MemoryStorage::default());
        let mut storage = Storage::Memory(glue);

        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
    }

    pub async fn file(path: &str) -> Result<Self> {
        let mut storage = FileStorage::new(path).map(Glue::new).map(Storage::File)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
    }

    pub async fn redb(path: &str) -> Result<Self> {
        if let Some(parent) = std::path::Path::new(path).parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)
                .map_err(|e| Error::BackendError(format!("failed to create directory: {e}")))?;
        }

        let mut storage = RedbStorage::new(path).map(Glue::new).map(Storage::Redb)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
    }

    pub async fn git(path: &str, remote: String, branch: String) -> Result<Self> {
        let mut storage = GitStorage::open(path, StorageType::File)?;
        storage.set_remote(remote);
        storage.set_branch(branch);

        let mut storage = Storage::Git(Glue::new(storage));
        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
    }

    pub async fn mongo(conn_str: &str, db_name: &str) -> Result<Self> {
        let mut storage = MongoStorage::new(conn_str, db_name)
            .await
            .map(Glue::new)
            .map(Storage::Mongo)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self { storage, root_id })
    }

    pub fn sync_job(&self) -> Option<SyncJob> {
        if let Storage::Git(glue) = &self.storage {
            Some(SyncJob::Git {
                path: glue.storage.path.clone(),
                remote: glue.storage.remote.clone(),
                branch: glue.storage.branch.clone(),
            })
        } else {
            None
        }
    }
}

#[async_trait]
pub trait Execute
where
    Self: Sized,
{
    async fn execute(self, storage: &mut Storage) -> Result<Payload>;
}

#[async_trait]
impl<T: Build + Send> Execute for T
where
    Self: Sized,
{
    async fn execute(self, storage: &mut Storage) -> Result<Payload> {
        let statement = self.build()?;

        match storage {
            Storage::Memory(glue) => glue.execute_stmt(&statement).await,
            Storage::File(glue) => glue.execute_stmt(&statement).await,
            Storage::Redb(glue) => glue.execute_stmt(&statement).await,
            Storage::Git(glue) => glue.execute_stmt(&statement).await,
            Storage::Mongo(glue) => glue.execute_stmt(&statement).await,
        }
        .map_err(Into::into)
    }
}
