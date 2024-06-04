use {
    async_io::block_on,
    gluesql::{
        core::ast_builder::{table, text, Execute},
        prelude::{CsvStorage, Glue},
    },
    std::{
        fmt::{Debug, Display},
        future::Future,
    },
};

const PATH: &str = ".glues/";

fn get_glue() -> Glue<CsvStorage> {
    let storage = CsvStorage::new(PATH).unwrap();

    Glue::new(storage)
}

pub fn init() {
    block_on(async {
        let mut glue = get_glue();

        table("logs")
            .drop_table_if_exists()
            .execute(&mut glue)
            .await
            .unwrap();

        table("logs")
            .create_table_if_not_exists()
            .add_column("timestamp TIMESTAMP DEFAULT NOW()")
            .add_column("message TEXT")
            .execute(&mut glue)
            .await
            .unwrap();
    })
}

pub fn log(message: &str) {
    block_on(async {
        let mut glue = get_glue();

        table("logs")
            .insert()
            .columns("message")
            .values(vec![vec![text(message)]])
            .execute(&mut glue)
            .await
            .unwrap();
    })
}

pub trait LogExpectExt<V> {
    fn log_expect(self, message: &str) -> V;
}

impl<V> LogExpectExt<V> for Option<V> {
    fn log_expect(self, message: &str) -> V {
        if let Some(v) = self {
            v
        } else {
            log(message);
            panic!("{message}");
        }
    }
}

#[allow(dead_code)]
pub trait LogUnwrapExt<V> {
    fn log_unwrap(self) -> V;
}

impl<V, E> LogUnwrapExt<V> for Result<V, E>
where
    E: Debug + Display,
{
    fn log_unwrap(self) -> V {
        match self {
            Ok(v) => v,
            Err(e) => {
                let e = e.to_string();
                log(&e);
                panic!("{e}");
            }
        }
    }
}

pub trait LogFutureUnwrapExt<V> {
    fn log_unwrap(self) -> V;
}

impl<V, E, F> LogFutureUnwrapExt<V> for F
where
    F: Future<Output = Result<V, E>>,
    E: Debug + Display,
{
    fn log_unwrap(self) -> V {
        match block_on(self) {
            Ok(v) => v,
            Err(e) => {
                let e = e.to_string();
                log(&e);
                panic!("{e}");
            }
        }
    }
}
