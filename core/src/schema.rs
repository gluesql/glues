use {
    crate::{
        Error, Result,
        db::{Execute, Storage, get_str},
        types::DirectoryId,
    },
    gluesql::core::ast_builder::{col, table, text},
};

pub async fn setup(storage: &mut Storage) -> Result<DirectoryId> {
    table("Log")
        .create_table_if_not_exists()
        .add_column("category TEXT NULL")
        .add_column("message TEXT NOT NULL")
        .add_column("created_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .execute(storage)
        .await?;

    table("Log").delete().execute(storage).await?;

    table("Meta")
        .create_table_if_not_exists()
        .add_column("key TEXT PRIMARY KEY")
        .add_column("value TEXT NOT NULL")
        .add_column("updated_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .execute(storage)
        .await?;

    table("Directory")
        .create_table_if_not_exists()
        .add_column("id UUID PRIMARY KEY DEFAULT GENERATE_UUID()")
        .add_column("parent_id UUID NULL")
        .add_column("name TEXT NOT NULL")
        .add_column("created_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .add_column("updated_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .execute(storage)
        .await?;

    table("Note")
        .create_table_if_not_exists()
        .add_column("id UUID PRIMARY KEY")
        .add_column("name TEXT NOT NULL")
        .add_column("directory_id UUID NOT NULL")
        .add_column("created_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .add_column("updated_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .add_column("content TEXT NOT NULL DEFAULT ''")
        .execute(storage)
        .await?;

    let schema_version_not_exists = table("Meta")
        .select()
        .filter(col("key").eq(text("schema_version")))
        .project("key")
        .execute(storage)
        .await?
        .select()
        .ok_or(Error::NotFound("schema_version not found".to_owned()))?
        .count()
        == 0;

    if schema_version_not_exists {
        table("Meta")
            .insert()
            .columns(vec!["key", "value"])
            .values(vec![vec![text("schema_version"), text("1")]])
            .execute(storage)
            .await?;
    }

    let root_not_exists = table("Directory")
        .select()
        .filter(col("parent_id").is_null())
        .project("id")
        .execute(storage)
        .await?
        .select()
        .ok_or(Error::NotFound("root directory not found".to_owned()))?
        .count()
        == 0;

    if root_not_exists {
        table("Directory")
            .insert()
            .columns("name")
            .values(vec![vec![text("Notes")]])
            .execute(storage)
            .await?;
    }

    let payload = table("Directory")
        .select()
        .filter(col("parent_id").is_null())
        .project("id")
        .execute(storage)
        .await?;

    let mut rows = payload
        .select()
        .ok_or(Error::NotFound("root directory not found".to_owned()))?;

    let row = rows
        .next()
        .ok_or(Error::NotFound("root directory not found".to_owned()))?;

    get_str(&row, "id")
}
