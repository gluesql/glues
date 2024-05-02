use gluesql::{
    core::ast_builder::{table, Execute},
    prelude::{Glue, MemoryStorage},
};

pub async fn setup(glue: &mut Glue<MemoryStorage>) {
    table("Directory")
        .create_table()
        .add_column("id UUID PRIMARY KEY DEFAULT GENERATE_UUID()")
        .add_column("parent_id UUID NULL")
        .add_column("name TEXT NOT NULL")
        .add_column("created_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .execute(glue)
        .await
        .expect("Creating Directory failed");

    table("Note")
        .create_table()
        .add_column("id UUID PRIMARY KEY DEFAULT GENERATE_UUID()")
        .add_column("name TEXT NOT NULL")
        .add_column("directory_id UUID NOT NULL")
        .add_column("created_at TIMESTAMP NOT NULL DEFAULT NOW()")
        .add_column("content TEXT NOT NULL DEFAULT 'blank'")
        .execute(glue)
        .await
        .expect("Creating Note failed");
}
