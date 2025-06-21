use {
    super::{Db, Execute},
    crate::Result,
    gluesql::core::ast_builder::{table, text},
};

impl Db {
    pub async fn log(&mut self, category: String, message: String) -> Result<()> {
        table("Log")
            .insert()
            .columns(vec!["category", "message"])
            .values(vec![vec![text(category), text(message)]])
            .execute(&mut self.storage)
            .await?;

        Ok(())
    }
}
