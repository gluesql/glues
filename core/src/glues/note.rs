use {
    crate::{
        data::Note,
        types::{DirectoryId, NoteId},
        Error, Glues, Result,
    },
    gluesql::core::ast_builder::{col, function::now, table, text, uuid, Execute},
    std::ops::Deref,
    uuid::Uuid,
};

// fetch
impl Glues {
    pub async fn fetch_note_content(&mut self, note_id: NoteId) -> Result<String> {
        let content = table("Note")
            .select()
            .filter(col("id").eq(uuid(note_id)))
            .project(col("content"))
            .execute(&mut self.glue)
            .await?
            .select()
            .ok_or(Error::Wip("error case 2".to_owned()))?
            .next()
            .ok_or(Error::Wip("error case 3".to_owned()))?
            .get("content")
            .map(Deref::deref)
            .ok_or(Error::Wip("error case 4".to_owned()))?
            .into();

        Ok(content)
    }

    pub async fn fetch_notes(&mut self, directory_id: DirectoryId) -> Result<Vec<Note>> {
        let notes = table("Note")
            .select()
            .filter(col("directory_id").eq(uuid(directory_id.clone())))
            .project(vec!["id", "name"])
            .execute(&mut self.glue)
            .await?
            .select()
            .unwrap()
            .map(|payload| Note {
                id: payload.get("id").map(Deref::deref).unwrap().into(),
                directory_id: directory_id.clone(),
                name: payload.get("name").map(Deref::deref).unwrap().into(),
            })
            .collect();

        Ok(notes)
    }

    pub async fn add_note(&mut self, directory_id: DirectoryId, name: String) -> Result<()> {
        let id = Uuid::new_v4().to_string();

        table("Note")
            .insert()
            .columns(vec!["id", "directory_id", "name"])
            .values(vec![vec![uuid(id.clone()), uuid(directory_id), text(name)]])
            .execute(&mut self.glue)
            .await?;

        Ok(())
    }

    pub async fn remove_note(&mut self, note_id: NoteId) -> Result<()> {
        table("Note")
            .delete()
            .filter(col("id").eq(uuid(note_id)))
            .execute(&mut self.glue)
            .await?;

        Ok(())
    }

    pub async fn update_note_content(&mut self, note_id: NoteId, content: String) {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("content", content)
            .set("updated_at", now())
            .execute(&mut self.glue)
            .await
            .unwrap();
    }

    pub async fn rename_note(&mut self, note_id: NoteId, name: String) {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("name", name)
            .set("updated_at", now())
            .execute(&mut self.glue)
            .await
            .unwrap();
    }

    pub async fn move_note(&mut self, note_id: NoteId, directory_id: DirectoryId) {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("directory_id", directory_id)
            .set("updated_at", now())
            .execute(&mut self.glue)
            .await
            .unwrap();
    }
}
