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
}

impl From<NoteRow> for Note {
    fn from(row: NoteRow) -> Self {
        Self {
            id: row.id,
            directory_id: row.directory_id,
            name: row.name,
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
        let result = table("Note")
            .select()
            .filter(col("directory_id").eq(uuid(directory_id.clone())))
            .project(vec!["id", "directory_id", "name"])
            .execute(&mut self.storage)
            .await?;

        let notes = result
            .rows_as::<NoteRow>()?
            .into_iter()
            .map(Note::from)
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
}
