mod directory;
mod note;
mod note_tree;

use {
    crate::{
        schema::setup,
        state::{EntryState, State},
        types::DirectoryId,
        Event, Result,
    },
    gluesql::{
        core::ast_builder::{col, table, text, Execute},
        prelude::{Glue, MemoryStorage},
    },
    note_tree::NoteTree,
    std::ops::Deref,
};

pub struct Glues {
    glue: Glue<MemoryStorage>,
    pub root_id: DirectoryId,
    pub state: State,
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
            .values(vec![vec![text("Notes")]])
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
            state: State::Entry(EntryState),
        }
    }

    pub async fn dispatch(&mut self, event: Event) -> Result<()> {
        State::consume(self, event).await
    }
}
