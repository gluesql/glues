pub type NoteId = String; // UUID
pub type DirectoryId = String; // UUID
pub type Id = String; // UUID

#[derive(Clone, Debug)]
pub struct KeymapItem {
    pub key: String,
    pub desc: String,
}

impl KeymapItem {
    pub fn new<K: Into<String>, D: Into<String>>(key: K, desc: D) -> Self {
        Self {
            key: key.into(),
            desc: desc.into(),
        }
    }
}
