use {
    crate::{
        data::{Directory, Note},
        types::DirectoryId,
        Glues,
    },
    std::collections::HashMap,
};

pub struct EntryNode {
    glues: Glues,
    notes_map: HashMap<DirectoryId, DirectoryItem>,
}

impl EntryNode {
    pub async fn new() -> Self {
        let mut glues = Glues::new().await;

        let data = glues.fetch_directory(glues.root_id.clone()).await;
        let directories = glues.fetch_directories(glues.root_id.clone()).await;
        let notes = glues.fetch_notes(glues.root_id.clone()).await;

        let directory_item = DirectoryItem {
            data,
            directories: Some(directories),
            notes: Some(notes),
        };
        let notes_map = [(glues.root_id.clone(), directory_item)].into();

        Self { glues, notes_map }
    }

    pub async fn open_directory(self, _directory_id: DirectoryId) -> Self {
        self
    }

    pub async fn close_directory(self, _directory_id: DirectoryId) -> Self {
        self
    }
}

pub struct DirectoryItem {
    data: Directory,
    directories: Option<Vec<Directory>>,
    notes: Option<Vec<Note>>,
}

impl From<Directory> for DirectoryItem {
    fn from(data: Directory) -> Self {
        Self {
            data,
            directories: None,
            notes: None,
        }
    }
}
