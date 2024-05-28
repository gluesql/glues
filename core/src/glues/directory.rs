use {
    crate::{data::Directory, types::DirectoryId, Glues},
    gluesql::core::ast_builder::{col, function::now, table, text, uuid, Execute},
    std::ops::Deref,
    uuid::Uuid,
};

impl Glues {
    pub async fn fetch_directory(&mut self, directory_id: DirectoryId) -> Directory {
        let directory = table("Directory")
            .select()
            .filter(col("id").eq(uuid(directory_id)))
            .project(vec!["id", "parent_id", "name"])
            .execute(&mut self.glue)
            .await
            .unwrap()
            .select()
            .unwrap()
            .next()
            .map(|payload| Directory {
                id: payload.get("id").map(Deref::deref).unwrap().into(),
                parent_id: payload.get("parent_id").map(Deref::deref).unwrap().into(),
                name: payload.get("name").map(Deref::deref).unwrap().into(),
            })
            .unwrap();

        directory
    }

    pub async fn fetch_directories(&mut self, parent_id: DirectoryId) -> Vec<Directory> {
        let directories = table("Directory")
            .select()
            .filter(col("parent_id").eq(uuid(parent_id.clone())))
            .project(vec!["id", "name"])
            .execute(&mut self.glue)
            .await
            .unwrap()
            .select()
            .unwrap()
            .map(|payload| Directory {
                id: payload.get("id").map(Deref::deref).unwrap().into(),
                parent_id: parent_id.clone(),
                name: payload.get("name").map(Deref::deref).unwrap().into(),
            })
            .collect();

        directories
    }

    pub async fn add_directory(&mut self, parent_id: DirectoryId, name: String) -> DirectoryId {
        let id = Uuid::new_v4().to_string();

        table("Directory")
            .insert()
            .columns(vec!["id", "parent_id", "name"])
            .values(vec![vec![uuid(id.clone()), uuid(parent_id), text(name)]])
            .execute(&mut self.glue)
            .await
            .unwrap();

        id
    }

    pub async fn remove_directory(&mut self, directory_id: DirectoryId) {
        table("Directory")
            .delete()
            .filter(col("id").eq(uuid(directory_id)))
            .execute(&mut self.glue)
            .await
            .unwrap();
    }

    pub async fn move_directory(&mut self, directory_id: DirectoryId, parent_id: DirectoryId) {
        table("Directory")
            .update()
            .filter(col("directory_id").eq(uuid(directory_id)))
            .set("parent_id", parent_id)
            .execute(&mut self.glue)
            .await
            .unwrap();
    }

    pub async fn rename_directory(&mut self, directory_id: DirectoryId, name: String) {
        table("Directory")
            .update()
            .filter(col("directory_id").eq(uuid(directory_id)))
            .set("name", name)
            .set("updated_at", now())
            .execute(&mut self.glue)
            .await
            .unwrap();
    }
}
