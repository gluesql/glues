mod directory;
mod note;
mod note_tree;

use note_tree::NoteTree;

use {
    crate::{schema::setup, types::DirectoryId},
    gluesql::{
        core::ast_builder::{col, table, text, Execute},
        prelude::{Glue, MemoryStorage},
    },
    std::ops::Deref,
};

pub struct Glues {
    glue: Glue<MemoryStorage>,
    pub root_id: DirectoryId,
    note_tree: NoteTree,
}

impl Glues {
    pub async fn new() -> Self {
        let storage = MemoryStorage::default();
        let mut glue = Glue::new(storage);

        setup(&mut glue).await;

        table("Directory")
            .insert()
            .columns("name")
            .values(vec![vec![text("root")]])
            .execute(&mut glue)
            .await
            .unwrap();

        let root_id = table("Directory")
            .select()
            .filter(col("parent_id").is_null())
            .project("id")
            .execute(&mut glue)
            .await
            .unwrap()
            .select()
            .unwrap()
            .next()
            .unwrap()
            .get("id")
            .map(Deref::deref)
            .unwrap()
            .into();

        println!("root id: {root_id}");
        let note_tree = NoteTree::default();

        Self {
            glue,
            root_id,
            note_tree,
        }
    }
}
