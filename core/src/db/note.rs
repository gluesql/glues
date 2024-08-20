use {
    super::{Db, Execute},
    crate::{
        data::Note,
        types::{DirectoryId, NoteId},
        Error, Result,
    },
    gluesql::core::ast_builder::{col, function::now, table, text, uuid},
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
            .execute(&mut self.storage)
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

    pub async fn add_note(&mut self, directory_id: DirectoryId, name: String) -> Result<Note> {
        let id = Uuid::now_v7().to_string();
        let note = Note {
            id: id.clone(),
            directory_id: directory_id.clone(),
            name: name.clone(),
        };

        table("Note")
            .insert()
            .columns(vec!["id", "directory_id", "name"])
            .values(vec![vec![uuid(id), uuid(directory_id), text(name)]])
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

    pub async fn move_note(&mut self, note_id: NoteId, directory_id: DirectoryId) {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("directory_id", directory_id)
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await
            .unwrap();
    }
}
