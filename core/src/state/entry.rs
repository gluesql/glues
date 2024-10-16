use crate::{
    db::Db, state::notebook::NotebookState, EntryEvent, EntryTransition, Error, Event, Glues,
    Result,
};

pub struct EntryState;

impl EntryState {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<EntryTransition> {
        use EntryEvent::*;
        use Event::*;

        match event {
            Entry(OpenMemory) => {
                let mut db = Db::memory(glues.task_tx.clone()).await?;
                let root_id = db.root_id.clone();
                let note_id = db.add_note(root_id, "Sample Note".to_owned()).await?.id;
                db.update_note_content(note_id, "Hi :D".to_owned()).await?;

                // test
                let dir_id = db
                    .add_directory(db.root_id.clone(), "Sample Directory".to_owned())
                    .await?
                    .id;
                db.add_note(dir_id.clone(), "Sub Note".to_owned()).await?;
                let dir_id = db.add_directory(dir_id, "sub sub dir".to_owned()).await?.id;
                db.add_note(dir_id, "Sub Sub Note".to_owned()).await?;
                db.add_directory(db.root_id.clone(), "Second empty dir".to_owned())
                    .await?;

                glues.db = Some(db);
                glues.state = NotebookState::new(glues).await?.into();
                Ok(EntryTransition::OpenNotebook)
            }
            Entry(OpenCsv(path)) => {
                glues.db = Db::csv(glues.task_tx.clone(), &path).await.map(Some)?;
                glues.state = NotebookState::new(glues).await?.into();

                Ok(EntryTransition::OpenNotebook)
            }
            Entry(OpenJson(path)) => {
                glues.db = Db::json(glues.task_tx.clone(), &path).await.map(Some)?;
                glues.state = NotebookState::new(glues).await?.into();

                Ok(EntryTransition::OpenNotebook)
            }
            Entry(OpenFile(path)) => {
                glues.db = Db::file(glues.task_tx.clone(), &path).await.map(Some)?;
                glues.state = NotebookState::new(glues).await?.into();

                Ok(EntryTransition::OpenNotebook)
            }
            Entry(OpenGit {
                path,
                remote,
                branch,
            }) => {
                glues.db = Db::git(glues.task_tx.clone(), &path, remote, branch)
                    .await
                    .map(Some)?;
                glues.state = NotebookState::new(glues).await?.into();

                Ok(EntryTransition::OpenNotebook)
            }
            Key(_) => Ok(EntryTransition::Inedible(event)),
            Cancel => Ok(EntryTransition::None),
            _ => Err(Error::Wip("todo: EntryState::consume".to_owned())),
        }
    }

    pub fn describe(&self) -> Result<String> {
        Ok("Entry".to_owned())
    }

    pub fn shortcuts(&self) -> Vec<&str> {
        vec!["[j] Next", "[k] Previous", "[Enter] Select", "[q] Quit"]
    }
}
