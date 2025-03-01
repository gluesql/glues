use {
    crate::logger::*,
    gluesql::{
        core::ast_builder::{Execute, col, table, text},
        prelude::{CsvStorage, Glue},
    },
    home::home_dir,
    std::ops::Deref,
};

pub const LAST_CSV_PATH: &str = "last_csv_path";
pub const LAST_JSON_PATH: &str = "last_json_path";
pub const LAST_FILE_PATH: &str = "last_file_path";
pub const LAST_GIT_PATH: &str = "last_git_path";
pub const LAST_GIT_REMOTE: &str = "last_git_remote";
pub const LAST_GIT_BRANCH: &str = "last_git_branch";
pub const LAST_MONGO_CONN_STR: &str = "last_mongo_conn_str";
pub const LAST_MONGO_DB_NAME: &str = "last_mongo_db_name";

const PATH: &str = ".glues/";

pub fn get_glue() -> Glue<CsvStorage> {
    let path = home_dir()
        .unwrap_or(std::env::current_dir().expect("failed to get current directory"))
        .join(PATH);
    let storage = CsvStorage::new(path).unwrap();

    Glue::new(storage)
}

pub async fn init() {
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
        (LAST_GIT_REMOTE, "origin"),
        (LAST_GIT_BRANCH, "main"),
        (LAST_MONGO_CONN_STR, ""),
        (LAST_MONGO_DB_NAME, ""),
    ] {
        let _ = table("config")
            .insert()
            .columns(vec!["key", "value"])
            .values(vec![vec![text(key), text(value)]])
            .execute(&mut glue)
            .await;
    }
}

pub async fn update(key: &str, value: &str) {
    let mut glue = get_glue();

    table("config")
        .update()
        .filter(col("key").eq(text(key)))
        .set("value", text(value))
        .execute(&mut glue)
        .await
        .unwrap();
}

pub async fn get(key: &str) -> Option<String> {
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
}
