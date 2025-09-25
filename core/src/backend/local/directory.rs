use {
    super::{Db, Execute},
    crate::{Result, data::Directory, types::DirectoryId},
    async_recursion::async_recursion,
    gluesql::{
        FromGlueRow,
        core::{
            ast_builder::{col, function::now, table, text, uuid},
            row_conversion::SelectExt,
        },
    },
    uuid::Uuid,
};

#[derive(FromGlueRow)]
struct DirectoryRow {
    id: String,
    parent_id: Option<String>,
    name: String,
}

impl DirectoryRow {
    /// Convert a raw row into a `Directory`, normalizing a NULL `parent_id`
    /// (the root entry) so it points back at its own `id`. Downstream logic
    /// already detects the root via `id` equality checks, so keeping
    /// `parent_id` populated avoids sprinkling Options everywhere.
    fn into_directory(self) -> Directory {
        let DirectoryRow {
            id,
            parent_id,
            name,
        } = self;
        let parent_id = parent_id.unwrap_or_else(|| id.clone());

        Directory {
            id,
            parent_id,
            name,
        }
    }
}

impl From<DirectoryRow> for Directory {
    fn from(row: DirectoryRow) -> Self {
        row.into_directory()
    }
}

impl Db {
    pub async fn fetch_directory(&mut self, directory_id: DirectoryId) -> Result<Directory> {
        let row = table("Directory")
            .select()
            .filter(col("id").eq(uuid(directory_id)))
            .project(vec!["id", "parent_id", "name"])
            .execute(&mut self.storage)
            .await?
            .one_as::<DirectoryRow>()?;

        Ok(Directory::from(row))
    }

    pub async fn fetch_directories(&mut self, parent_id: DirectoryId) -> Result<Vec<Directory>> {
        let result = table("Directory")
            .select()
            .filter(col("parent_id").eq(uuid(parent_id)))
            .project(vec!["id", "parent_id", "name"])
            .execute(&mut self.storage)
            .await?;

        Ok(result
            .rows_as::<DirectoryRow>()?
            .into_iter()
            .map(Directory::from)
            .collect())
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

    #[cfg_attr(target_arch = "wasm32", async_recursion(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_recursion)]
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
