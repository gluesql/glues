use {
    crate::{data::Directory, types::DirectoryId, Glues, Result},
    gluesql::core::ast_builder::{col, function::now, table, text, uuid, Execute},
    std::ops::Deref,
    uuid::Uuid,
};

impl Glues {
    pub async fn fetch_directory(&mut self, directory_id: DirectoryId) -> Result<Directory> {
        let directory = table("Directory")
            .select()
            .filter(col("id").eq(uuid(directory_id)))
            .project(vec!["id", "parent_id", "name"])
            .execute(&mut self.glue)
            .await?
            .select()
            .unwrap()
            .next()
            .map(|payload| Directory {
                id: payload.get("id").map(Deref::deref).unwrap().into(),
                parent_id: payload.get("parent_id").map(Deref::deref).unwrap().into(),
                name: payload.get("name").map(Deref::deref).unwrap().into(),
            })
            .unwrap();

        Ok(directory)
    }

    pub async fn fetch_directories(&mut self, parent_id: DirectoryId) -> Result<Vec<Directory>> {
        let directories = table("Directory")
            .select()
            .filter(col("parent_id").eq(uuid(parent_id.clone())))
            .project(vec!["id", "name"])
            .execute(&mut self.glue)
            .await?
            .select()
            .unwrap()
            .map(|payload| Directory {
                id: payload.get("id").map(Deref::deref).unwrap().into(),
                parent_id: parent_id.clone(),
                name: payload.get("name").map(Deref::deref).unwrap().into(),
            })
            .collect();

        Ok(directories)
    }

    pub async fn add_directory(
        &mut self,
        parent_id: DirectoryId,
        name: String,
    ) -> Result<Directory> {
        let id = Uuid::new_v4().to_string();
        let directory = Directory {
            id: id.clone(),
            parent_id: parent_id.clone(),
            name: name.clone(),
        };

        table("Directory")
            .insert()
            .columns(vec!["id", "parent_id", "name"])
            .values(vec![vec![uuid(id.clone()), uuid(parent_id), text(name)]])
            .execute(&mut self.glue)
            .await?;

        Ok(directory)
    }

    pub async fn remove_directory(&mut self, directory_id: DirectoryId) -> Result<()> {
        table("Directory")
            .delete()
            .filter(col("id").eq(uuid(directory_id)))
            .execute(&mut self.glue)
            .await?;

        Ok(())
    }

    pub async fn move_directory(
        &mut self,
        directory_id: DirectoryId,
        parent_id: DirectoryId,
    ) -> Result<()> {
        table("Directory")
            .update()
            .filter(col("directory_id").eq(uuid(directory_id)))
            .set("parent_id", parent_id)
            .execute(&mut self.glue)
            .await?;

        Ok(())
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
            .execute(&mut self.glue)
            .await?;

        Ok(())
    }
}
