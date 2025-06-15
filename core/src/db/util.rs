use {
    crate::{Error, Result},
    gluesql::prelude::Value,
    std::{collections::HashMap, ops::Deref},
};

pub(crate) fn get_str(payload: &HashMap<&str, &Value>, key: &str) -> Result<String> {
    payload
        .get(key)
        .map(Deref::deref)
        .ok_or_else(|| Error::NotFound(format!("{key} not found")))
        .map(Into::into)
}
