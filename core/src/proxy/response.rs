use crate::data::{Directory, Note};
use crate::types::DirectoryId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "status", content = "data")]
pub enum ProxyResponse {
    Ok(ResultPayload),
    Err(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum ResultPayload {
    Id(DirectoryId),
    Directory(Directory),
    Directories(Vec<Directory>),
    Note(Note),
    Notes(Vec<Note>),
    Text(String),
    Unit,
}
