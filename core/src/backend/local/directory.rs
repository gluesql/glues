use {
    super::{Db, Execute},
    crate::{Result, data::Directory, types::DirectoryId},
    async_recursion::async_recursion,
    gluesql::core::ast_builder::{col, function::now, num, table, text, uuid},
    std::ops::Deref,
    uuid::Uuid,
};

impl Db {
    pub async fn fetch_directory(&mut self, directory_id: DirectoryId) -> Result<Directory> {
        let directory = table("Directory")
            .select()
            .filter(col("id").eq(uuid(directory_id)))
            .project(vec!["id", "parent_id", "name", "order"])
            .execute(&mut self.storage)
            .await?
            .select()
            .unwrap()
            .next()
            .map(|payload| Directory {
                id: payload.get("id").map(Deref::deref).unwrap().into(),
                parent_id: payload.get("parent_id").map(Deref::deref).unwrap().into(),
                name: payload.get("name").map(Deref::deref).unwrap().into(),
                order: payload.get("order").cloned().unwrap().try_into().unwrap(),
            })
            .unwrap();

        Ok(directory)
    }

    pub async fn fetch_directories(&mut self, parent_id: DirectoryId) -> Result<Vec<Directory>> {
        let directories = table("Directory")
            .select()
            .filter(col("parent_id").eq(uuid(parent_id.clone())))
            .project(vec!["id", "name", "order"])
            .order_by("order")
            .execute(&mut self.storage)
            .await?
            .select()
            .unwrap()
            .map(|payload| Directory {
                id: payload.get("id").map(Deref::deref).unwrap().into(),
                parent_id: parent_id.clone(),
                name: payload.get("name").map(Deref::deref).unwrap().into(),
                order: payload.get("order").cloned().unwrap().try_into().unwrap(),
            })
            .collect();

        Ok(directories)
    }

    pub async fn add_directory(
        &mut self,
        parent_id: DirectoryId,
        name: String,
    ) -> Result<Directory> {
        let id = Uuid::now_v7().to_string();
        let order = self
            .fetch_directories(parent_id.clone())
            .await?
            .into_iter()
            .map(|d| d.order)
            .max()
            .unwrap_or(-1)
            + 1;

        let directory = Directory {
            id: id.clone(),
            parent_id: parent_id.clone(),
            name: name.clone(),
            order,
        };

        table("Directory")
            .insert()
            .columns(vec!["id", "parent_id", "name", "order"])
            .values(vec![vec![
                uuid(id.clone()),
                uuid(parent_id),
                text(name),
                num(order),
            ]])
            .execute(&mut self.storage)
            .await?;

        self.sync().map(|()| directory)
    }

    #[async_recursion(?Send)]
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

    pub async fn reorder_directory(&mut self, directory_id: DirectoryId, order: i64) -> Result<()> {
        table("Directory")
            .update()
            .filter(col("id").eq(uuid(directory_id)))
            .set("order", num(order))
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await?;

        self.sync()
    }
}
