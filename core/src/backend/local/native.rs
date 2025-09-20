use {
    crate::{Error, Result, schema::setup, task::Task, types::DirectoryId},
    async_trait::async_trait,
    gluesql::{
        core::ast_builder::Build,
        gluesql_git_storage::{GitStorage, StorageType},
        gluesql_mongo_storage::MongoStorage,
        gluesql_redb_storage::RedbStorage,
        prelude::{FileStorage, Glue, MemoryStorage, Payload},
    },
    std::sync::mpsc::Sender,
};

pub struct Db {
    pub storage: Storage,
    pub root_id: DirectoryId,
    pub task_tx: Sender<Task>,
}

pub enum Storage {
    Memory(Glue<MemoryStorage>),
    File(Glue<FileStorage>),
    Redb(Glue<RedbStorage>),
    Git(Glue<GitStorage>),
    Mongo(Glue<MongoStorage>),
}

impl Db {
    pub async fn memory(task_tx: Sender<Task>) -> Result<Self> {
        let glue = Glue::new(MemoryStorage::default());
        let mut storage = Storage::Memory(glue);

        let root_id = setup(&mut storage).await?;

        Ok(Self {
            storage,
            root_id,
            task_tx,
        })
    }

    pub async fn file(task_tx: Sender<Task>, path: &str) -> Result<Self> {
        let mut storage = FileStorage::new(path).map(Glue::new).map(Storage::File)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self {
            storage,
            root_id,
            task_tx,
        })
    }

    pub async fn redb(task_tx: Sender<Task>, path: &str) -> Result<Self> {
        if let Some(parent) = std::path::Path::new(path).parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)
                .map_err(|e| Error::BackendError(format!("failed to create directory: {e}")))?;
        }

        let mut storage = RedbStorage::new(path).map(Glue::new).map(Storage::Redb)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self {
            storage,
            root_id,
            task_tx,
        })
    }

    pub async fn git(
        task_tx: Sender<Task>,
        path: &str,
        remote: String,
        branch: String,
    ) -> Result<Self> {
        let mut storage = GitStorage::open(path, StorageType::File)?;
        storage.set_remote(remote);
        storage.set_branch(branch);

        let mut storage = Storage::Git(Glue::new(storage));
        let root_id = setup(&mut storage).await?;

        Ok(Self {
            storage,
            root_id,
            task_tx,
        })
    }

    pub async fn mongo(task_tx: Sender<Task>, conn_str: &str, db_name: &str) -> Result<Self> {
        let mut storage = MongoStorage::new(conn_str, db_name)
            .await
            .map(Glue::new)
            .map(Storage::Mongo)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self {
            storage,
            root_id,
            task_tx,
        })
    }

    pub fn pull(&mut self) -> Result<()> {
        if let Storage::Git(glue) = &mut self.storage {
            glue.storage.pull()?;
        }

        Ok(())
    }

    pub fn sync(&self) -> Result<()> {
        if let Storage::Git(glue) = &self.storage {
            let path = glue.storage.path.clone();
            let remote = glue.storage.remote.clone();
            let branch = glue.storage.branch.clone();

            let task = Task::GitSync {
                path,
                remote,
                branch,
            };

            self.task_tx
                .clone()
                .send(task)
                .map_err(|e| Error::BackendError(e.to_string()))?;
        }

        Ok(())
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
