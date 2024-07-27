use {
    crate::{
        db::{Db, Execute},
        schema::setup,
        state::{EntryState, State},
        types::DirectoryId,
        Event, Result, Transition,
    },
    gluesql::core::ast_builder::{col, table, text},
    std::ops::Deref,
};

pub struct Glues {
    pub db: Db,
    pub root_id: DirectoryId,
    pub state: State,
}

impl Glues {
    pub async fn new() -> Self {
        let mut db = Db::memory();

        setup(&mut db.storage).await;

        table("Directory")
            .insert()
            .columns("name")
            .values(vec![vec![text("Notes")]])
            .execute(&mut db.storage)
            .await
            .unwrap();

        let root_id = table("Directory")
            .select()
            .filter(col("parent_id").is_null())
            .project("id")
            .execute(&mut db.storage)
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

        Self {
            db,
            root_id,
            state: EntryState.into(),
        }
    }

    pub async fn dispatch(&mut self, event: Event) -> Result<Transition> {
        State::consume(self, event).await
    }
}
