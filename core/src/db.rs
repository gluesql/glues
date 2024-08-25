mod directory;
mod note;

use {
    crate::{schema::setup, task::Task, types::DirectoryId, Result},
    async_trait::async_trait,
    gluesql::{
        core::ast_builder::Build,
        gluesql_git_storage::{GitStorage, StorageType},
        prelude::{CsvStorage, FileStorage, Glue, JsonStorage, MemoryStorage, Payload},
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
    Csv(Glue<CsvStorage>),
    Json(Glue<JsonStorage>),
    File(Glue<FileStorage>),
    Git(Glue<GitStorage>),
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

    pub async fn csv(task_tx: Sender<Task>, path: &str) -> Result<Self> {
        let mut storage = CsvStorage::new(path).map(Glue::new).map(Storage::Csv)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self {
            storage,
            root_id,
            task_tx,
        })
    }

    pub async fn json(task_tx: Sender<Task>, path: &str) -> Result<Self> {
        let mut storage = JsonStorage::new(path).map(Glue::new).map(Storage::Json)?;

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

    pub async fn git(task_tx: Sender<Task>, path: &str) -> Result<Self> {
        let mut storage = GitStorage::open(path, StorageType::File)
            .map(Glue::new)
            .map(Storage::Git)?;

        let root_id = setup(&mut storage).await?;

        Ok(Self {
            storage,
            root_id,
            task_tx,
        })
    }

    pub async fn pull(&mut self) -> Result<()> {
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

            self.task_tx.clone().send(task).unwrap();
        }

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
            Storage::Git(glue) => glue.execute_stmt(&statement).await,
        }
        .map_err(Into::into)
    }
}
