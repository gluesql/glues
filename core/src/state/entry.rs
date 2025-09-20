use crate::{
    EntryEvent, EntryTransition, Error, Event, Glues, Result,
    backend::{local::Db, proxy::ProxyClient},
    state::notebook::NotebookState,
    types::{KeymapGroup, KeymapItem},
};

pub struct EntryState;

impl EntryState {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<EntryTransition> {
        use EntryEvent::*;
        use Event::*;

        match event {
            Entry(OpenMemory) => {
                let mut db = memory_backend(glues).await?;
                let root_id = db.root_id.clone();
                let note_id = db.add_note(root_id, "Sample Note".to_owned()).await?.id;
                db.update_note_content(note_id, "Hi :D".to_owned()).await?;

                glues.db = Some(Box::new(db));
                glues.state = NotebookState::new(glues).await?.into();
                Ok(EntryTransition::OpenNotebook)
            }
            #[cfg(not(target_arch = "wasm32"))]
            Entry(OpenFile(path)) => {
                let db = Db::file(glues.task_tx.clone(), &path).await?;
                glues.db = Some(Box::new(db));
                glues.state = NotebookState::new(glues).await?.into();

                Ok(EntryTransition::OpenNotebook)
            }
            #[cfg(not(target_arch = "wasm32"))]
            Entry(OpenGit {
                path,
                remote,
                branch,
            }) => {
                let db = Db::git(glues.task_tx.clone(), &path, remote, branch).await?;
                glues.db = Some(Box::new(db));
                glues.state = NotebookState::new(glues).await?.into();

                Ok(EntryTransition::OpenNotebook)
            }
            #[cfg(not(target_arch = "wasm32"))]
            Entry(OpenMongo { conn_str, db_name }) => {
                let db = Db::mongo(glues.task_tx.clone(), &conn_str, &db_name).await?;
                glues.db = Some(Box::new(db));
                glues.state = NotebookState::new(glues).await?.into();

                Ok(EntryTransition::OpenNotebook)
            }
            Entry(OpenProxy { url }) => {
                let client = ProxyClient::connect(url).await?;
                glues.db = Some(Box::new(client));
                glues.state = NotebookState::new(glues).await?.into();

                Ok(EntryTransition::OpenNotebook)
            }
            Key(_) => Ok(EntryTransition::Inedible(event)),
            Cancel => Ok(EntryTransition::None),
            _ => Err(Error::Todo("EntryState::consume".to_owned())),
        }
    }

    pub fn describe(&self) -> Result<String> {
        Ok(
            "Glues - TUI note-taking app offering complete data control and flexible storage options"
                .to_owned(),
        )
    }

    pub fn keymap(&self) -> Vec<KeymapGroup> {
        vec![
            KeymapGroup::new(
                "Navigation",
                vec![
                    KeymapItem::new("j", "Select next"),
                    KeymapItem::new("k", "Select previous"),
                ],
            ),
            KeymapGroup::new(
                "Actions",
                vec![
                    KeymapItem::new("Enter", "Run selected item"),
                    KeymapItem::new("q", "Quit"),
                ],
            ),
        ]
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn memory_backend(glues: &Glues) -> Result<Db> {
    Db::memory(glues.task_tx.clone()).await
}

#[cfg(target_arch = "wasm32")]
async fn memory_backend(_glues: &Glues) -> Result<Db> {
    Db::memory().await
}
