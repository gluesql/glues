use {
    super::{Db, Execute},
    crate::{
        Error, Result,
        data::Note,
        types::{DirectoryId, NoteId},
    },
    gluesql::core::ast_builder::{col, function::now, num, table, text, uuid},
    std::ops::Deref,
    uuid::Uuid,
};

impl Db {
    pub async fn fetch_note_content(&mut self, note_id: NoteId) -> Result<String> {
        let content = table("Note")
            .select()
            .filter(col("id").eq(uuid(note_id)))
            .project(col("content"))
            .execute(&mut self.storage)
            .await?
            .select()
            .ok_or(Error::NotFound("note not found".to_owned()))?
            .next()
            .ok_or(Error::NotFound("note not found".to_owned()))?
            .get("content")
            .map(Deref::deref)
            .ok_or(Error::NotFound("content not found".to_owned()))?
            .into();

        Ok(content)
    }

    pub async fn fetch_notes(&mut self, directory_id: DirectoryId) -> Result<Vec<Note>> {
        let notes = table("Note")
            .select()
            .filter(col("directory_id").eq(uuid(directory_id.clone())))
            .project(vec!["id", "name", "order"])
            .order_by("order")
            .execute(&mut self.storage)
            .await?
            .select()
            .unwrap()
            .map(|payload| Note {
                id: payload.get("id").map(Deref::deref).unwrap().into(),
                directory_id: directory_id.clone(),
                name: payload.get("name").map(Deref::deref).unwrap().into(),
                order: payload.get("order").cloned().unwrap().try_into().unwrap(),
            })
            .collect();

        Ok(notes)
    }

    pub async fn add_note(&mut self, directory_id: DirectoryId, name: String) -> Result<Note> {
        let id = Uuid::now_v7().to_string();
        let order = self
            .fetch_notes(directory_id.clone())
            .await?
            .into_iter()
            .map(|n| n.order)
            .max()
            .unwrap_or(-1)
            + 1;

        let note = Note {
            id: id.clone(),
            directory_id: directory_id.clone(),
            name: name.clone(),
            order,
        };

        table("Note")
            .insert()
            .columns(vec!["id", "directory_id", "name", "order"])
            .values(vec![vec![
                uuid(id),
                uuid(directory_id),
                text(name),
                num(order),
            ]])
            .execute(&mut self.storage)
            .await?;

        self.sync().map(|()| note)
    }

    pub async fn remove_note(&mut self, note_id: NoteId) -> Result<()> {
        table("Note")
            .delete()
            .filter(col("id").eq(uuid(note_id)))
            .execute(&mut self.storage)
            .await?;

        self.sync()
    }

    pub async fn update_note_content(&mut self, note_id: NoteId, content: String) -> Result<()> {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("content", text(content))
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await?;

        self.sync()
    }

    pub async fn rename_note(&mut self, note_id: NoteId, name: String) -> Result<()> {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("name", text(name))
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await?;

        self.sync()
    }

    pub async fn move_note(&mut self, note_id: NoteId, directory_id: DirectoryId) -> Result<()> {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("directory_id", uuid(directory_id))
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await?;

        self.sync()
    }

    pub async fn reorder_note(&mut self, note_id: NoteId, order: i64) -> Result<()> {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("order", num(order))
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await?;

        self.sync()
    }
}
