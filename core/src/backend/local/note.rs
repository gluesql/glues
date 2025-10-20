use {
    super::{Db, Execute},
    crate::{
        Result,
        data::Note,
        types::{DirectoryId, NoteId},
    },
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
struct NoteRow {
    id: String,
    directory_id: String,
    name: String,
    created_at: String,
    updated_at: String,
}

impl From<NoteRow> for Note {
    fn from(row: NoteRow) -> Self {
        Self {
            id: row.id,
            directory_id: row.directory_id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(FromGlueRow)]
struct NoteContentRow {
    content: String,
}

impl Db {
    pub async fn fetch_note_content(&mut self, note_id: NoteId) -> Result<String> {
        let row = table("Note")
            .select()
            .filter(col("id").eq(uuid(note_id)))
            .project(col("content"))
            .execute(&mut self.storage)
            .await?
            .one_as::<NoteContentRow>()?;

        Ok(row.content)
    }

    pub async fn fetch_notes(&mut self, directory_id: DirectoryId) -> Result<Vec<Note>> {
        let notes = table("Note")
            .select()
            .filter(col("directory_id").eq(uuid(directory_id)))
            .project(vec![
                "id",
                "directory_id",
                "name",
                "created_at",
                "updated_at",
            ])
            .execute(&mut self.storage)
            .await?
            .rows_as::<NoteRow>()?
            .into_iter()
            .map(Note::from)
            .collect();

        Ok(notes)
    }

    pub async fn add_note(&mut self, directory_id: DirectoryId, name: String) -> Result<Note> {
        let id = Uuid::now_v7().to_string();
        table("Note")
            .insert()
            .columns(vec!["id", "directory_id", "name"])
            .values(vec![vec![uuid(id.clone()), uuid(directory_id), text(name)]])
            .execute(&mut self.storage)
            .await?;

        Ok(table("Note")
            .select()
            .filter(col("id").eq(uuid(id)))
            .project(vec![
                "id",
                "directory_id",
                "name",
                "created_at",
                "updated_at",
            ])
            .execute(&mut self.storage)
            .await?
            .one_as::<NoteRow>()
            .map(Note::from)?)
    }

    pub async fn remove_note(&mut self, note_id: NoteId) -> Result<()> {
        table("Note")
            .delete()
            .filter(col("id").eq(uuid(note_id)))
            .execute(&mut self.storage)
            .await?;

        Ok(())
    }

    pub async fn update_note_content(&mut self, note_id: NoteId, content: String) -> Result<()> {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("content", text(content))
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await?;

        Ok(())
    }

    pub async fn rename_note(&mut self, note_id: NoteId, name: String) -> Result<()> {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("name", text(name))
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await?;

        Ok(())
    }

    pub async fn move_note(&mut self, note_id: NoteId, directory_id: DirectoryId) -> Result<()> {
        table("Note")
            .update()
            .filter(col("id").eq(uuid(note_id)))
            .set("directory_id", uuid(directory_id))
            .set("updated_at", now())
            .execute(&mut self.storage)
            .await?;

        Ok(())
    }
}
