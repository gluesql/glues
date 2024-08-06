use {
    crate::db::{Execute, Storage},
    gluesql::core::ast_builder::table,
};

pub async fn setup(storage: &mut Storage) {
    table("Directory")
        .create_table_if_not_exists()
        .add_column("id UUID PRIMARY KEY DEFAULT GENERATE_UUID()")
        .add_column("parent_id UUID NULL")
        .add_column("name TEXT NOT NULL")
        .add_column("created_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .add_column("updated_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .execute(storage)
        .await
        .expect("Creating Directory failed");

    table("Note")
        .create_table_if_not_exists()
        .add_column("id UUID PRIMARY KEY")
        .add_column("name TEXT NOT NULL")
        .add_column("directory_id UUID NOT NULL")
        .add_column("created_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .add_column("updated_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .add_column("content TEXT NOT NULL DEFAULT ''")
        .execute(storage)
        .await
        .expect("Creating Note failed");
}
