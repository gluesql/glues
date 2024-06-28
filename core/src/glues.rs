use {
    crate::{
        db::Db,
        schema::setup,
        state::{EntryState, State},
        types::DirectoryId,
        Event, Result,
    },
    gluesql::core::ast_builder::{col, table, text, Execute},
    std::ops::Deref,
};

pub struct Glues {
    pub db: Db,
    pub root_id: DirectoryId,
    pub state: State,
}

impl Glues {
    pub async fn new() -> Self {
        let mut db = Db::default();

        setup(&mut db.glue).await;

        table("Directory")
            .insert()
            .columns("name")
            .values(vec![vec![text("Notes")]])
            .execute(&mut db.glue)
            .await
            .unwrap();

        let root_id = table("Directory")
            .select()
            .filter(col("parent_id").is_null())
            .project("id")
            .execute(&mut db.glue)
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
            state: State::Entry(EntryState),
        }
    }

    pub async fn dispatch(&mut self, event: Event) -> Result<()> {
        State::consume(self, event).await
    }
}
