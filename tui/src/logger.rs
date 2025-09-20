use std::fmt::{Debug, Display};

#[cfg(target_arch = "wasm32")]
use ratzilla::web_sys::console;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(not(target_arch = "wasm32"))]
use {
    crate::config::platform::get_glue,
    gluesql::core::ast_builder::{Execute, table, text},
};

pub async fn init() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut glue = get_glue();
        table("logs")
            .create_table_if_not_exists()
            .add_column("timestamp TIMESTAMP DEFAULT NOW()")
            .add_column("message TEXT")
            .execute(&mut glue)
            .await
            .expect("logger should create logs table");
    }
}

pub async fn log(message: &str) {
    #[cfg(target_arch = "wasm32")]
    console::log_1(&JsValue::from_str(message));

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut glue = get_glue();
        table("logs")
            .insert()
            .columns("message")
            .values(vec![vec![text(message)]])
            .execute(&mut glue)
            .await
            .expect("logger should insert log entry");
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logger::log(&format!($($arg)*)).await;
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
            Err(e) => panic!("{e}"),
        }
    }
}
