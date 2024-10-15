use {
    crate::logger::*,
    async_io::block_on,
    gluesql::{
        core::ast_builder::{col, table, text, Execute},
        prelude::{CsvStorage, Glue},
    },
    home::home_dir,
    std::ops::Deref,
};

pub const LAST_CSV_PATH: &str = "last_csv_path";
pub const LAST_JSON_PATH: &str = "last_json_path";
pub const LAST_FILE_PATH: &str = "last_file_path";
pub const LAST_GIT_PATH: &str = "last_git_path";
pub const LAST_GIT_ORIGIN: &str = "last_git_origin";
pub const LAST_GIT_BRANCH: &str = "last_git_branch";

const PATH: &str = ".glues/";

pub fn get_glue() -> Glue<CsvStorage> {
    let path = home_dir()
        .unwrap_or(std::env::current_dir().expect("failed to get current directory"))
        .join(PATH);
    let storage = CsvStorage::new(path).unwrap();

    Glue::new(storage)
}

pub fn init() {
    block_on(async {
        let mut glue = get_glue();

        table("config")
            .create_table_if_not_exists()
            .add_column("key TEXT PRIMARY KEY")
            .add_column("value TEXT NOT NULL")
            .execute(&mut glue)
            .await
            .unwrap();

        for (key, value) in [
            (LAST_CSV_PATH, ""),
            (LAST_JSON_PATH, ""),
            (LAST_FILE_PATH, ""),
            (LAST_GIT_PATH, ""),
            (LAST_GIT_ORIGIN, ""),
            (LAST_GIT_BRANCH, ""),
        ] {
            let _ = table("config")
                .insert()
                .columns(vec!["key", "value"])
                .values(vec![vec![text(key), text(value)]])
                .execute(&mut glue)
                .await;
        }
    })
}

pub fn update(key: &str, value: &str) {
    block_on(async {
        let mut glue = get_glue();

        table("config")
            .update()
            .filter(col("key").eq(text(key)))
            .set("value", text(value))
            .execute(&mut glue)
            .await
            .unwrap();
    })
}

pub fn get(key: &str) -> Option<String> {
    block_on(async {
        let mut glue = get_glue();

        let value = table("config")
            .select()
            .filter(col("key").eq(text(key)))
            .project(col("value"))
            .execute(&mut glue)
            .await
            .log_unwrap()
            .select()
            .log_expect("payload is not from select query")
            .next()?
            .get("value")
            .map(Deref::deref)
            .log_expect("value does not exist in row")
            .into();

        Some(value)
    })
}
