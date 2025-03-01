use {
    super::config::get_glue,
    gluesql::core::ast_builder::{Execute, table, text},
    std::fmt::{Debug, Display},
};

pub async fn init() {
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
}

pub async fn log(message: &str) {
    let mut glue = get_glue();

    table("logs")
        .insert()
        .columns("message")
        .values(vec![vec![text(message)]])
        .execute(&mut glue)
        .await
        .unwrap();
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        log(&format!($($arg)*)).await;
    };
}

pub trait LogExpectExt<V> {
    fn log_expect(self, message: &str) -> V;
}

impl<V> LogExpectExt<V> for Option<V> {
    fn log_expect(self, message: &str) -> V {
        if let Some(v) = self {
            v
        } else {
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
                panic!("{e}");
            }
        }
    }
}
