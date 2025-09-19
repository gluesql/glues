use {
    super::{Db, Execute},
    crate::{Error, Result, data::Directory, types::DirectoryId},
    async_recursion::async_recursion,
    gluesql::core::ast_builder::{col, function::now, table, text, uuid},
    std::ops::Deref,
    uuid::Uuid,
};

impl Db {
    pub async fn fetch_directory(&mut self, directory_id: DirectoryId) -> Result<Directory> {
        let result = table("Directory")
            .select()
            .filter(col("id").eq(uuid(directory_id)))
            .project(vec!["id", "parent_id", "name"])
            .execute(&mut self.storage)
            .await?;

        let payload = result
            .select()
            .ok_or_else(|| Error::BackendError("expected select payload".into()))?
            .next()
            .ok_or_else(|| Error::BackendError("directory not found".into()))?;

        let directory = Directory {
            id: payload
                .get("id")
                .map(Deref::deref)
                .ok_or_else(|| Error::BackendError("id missing".into()))?
                .into(),
            parent_id: payload
                .get("parent_id")
                .map(Deref::deref)
                .ok_or_else(|| Error::BackendError("parent_id missing".into()))?
                .into(),
            name: payload
                .get("name")
                .map(Deref::deref)
                .ok_or_else(|| Error::BackendError("name missing".into()))?
                .into(),
        };

        Ok(directory)
    }

    pub async fn fetch_directories(&mut self, parent_id: DirectoryId) -> Result<Vec<Directory>> {
        let result = table("Directory")
            .select()
            .filter(col("parent_id").eq(uuid(parent_id.clone())))
            .project(vec!["id", "name"])
            .execute(&mut self.storage)
            .await?;

        let rows = result
            .select()
            .ok_or_else(|| Error::BackendError("expected select payload".into()))?;

        let directories = rows
            .map(|payload| {
                Ok(Directory {
                    id: payload
                        .get("id")
                        .map(Deref::deref)
                        .ok_or_else(|| Error::BackendError("id missing".into()))?
                        .into(),
                    parent_id: parent_id.clone(),
                    name: payload
                        .get("name")
                        .map(Deref::deref)
                        .ok_or_else(|| Error::BackendError("name missing".into()))?
                        .into(),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(directories)
    }

    pub async fn add_directory(
        &mut self,
        parent_id: DirectoryId,
        name: String,
    ) -> Result<Directory> {
        let id = Uuid::now_v7().to_string();
        let directory = Directory {
            id: id.clone(),
            parent_id: parent_id.clone(),
            name: name.clone(),
        };

        table("Directory")
            .insert()
            .columns(vec!["id", "parent_id", "name"])
            .values(vec![vec![uuid(id.clone()), uuid(parent_id), text(name)]])
            .execute(&mut self.storage)
            .await?;

        self.sync().map(|()| directory)
    }

    #[async_recursion]
    pub async fn remove_directory(&mut self, directory_id: DirectoryId) -> Result<()> {
        table("Note")
            .delete()
            .filter(col("directory_id").eq(uuid(directory_id.clone())))
            .execute(&mut self.storage)
            .await?;

        let directories = self.fetch_directories(directory_id.clone()).await?;
        for directory in directories {
            self.remove_directory(directory.id).await?;
        }

        table("Directory")
            .delete()
            .filter(col("id").eq(uuid(directory_id)))
            .execute(&mut self.storage)
            .await?;

        self.sync()
    }

    pub async fn move_directory(
        &mut self,
        directory_id: DirectoryId,
        parent_id: DirectoryId,
    ) -> Result<()> {
        table("Directory")
            .update()
            .filter(col("id").eq(uuid(directory_id)))
            .set("parent_id", uuid(parent_id))
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await?;

        self.sync()
    }

    pub async fn rename_directory(
        &mut self,
        directory_id: DirectoryId,
        name: String,
    ) -> Result<()> {
        table("Directory")
            .update()
            .filter(col("id").eq(uuid(directory_id)))
            .set("name", text(name))
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await?;

        self.sync()
    }
}
