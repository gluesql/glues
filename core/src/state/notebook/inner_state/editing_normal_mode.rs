use crate::{
    db::Db,
    state::notebook::{directory, note, NotebookState},
    Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
};

pub async fn consume(
    db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NotebookEvent::*;

    match event {
        Notebook(SelectNote(note)) => note::select(state, note),
        Notebook(SelectDirectory(directory)) => directory::select(state, directory),
        Notebook(UpdateNoteContent(content)) => note::update_content(db, state, content).await,
        Key(KeyEvent::E) | Notebook(EditNote) => note::edit(state).await,
        Key(KeyEvent::B) | Notebook(BrowseNoteTree) => note::browse(state).await,
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
