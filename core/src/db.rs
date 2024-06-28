mod directory;
mod note;
mod note_tree;

use gluesql::prelude::{Glue, MemoryStorage};

pub struct Db {
    pub glue: Glue<MemoryStorage>,
}

impl Default for Db {
    fn default() -> Self {
        let storage = MemoryStorage::default();
        let glue = Glue::new(storage);

        Self { glue }
    }
}
