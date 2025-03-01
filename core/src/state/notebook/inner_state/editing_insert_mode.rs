use crate::{
    Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
    db::Db,
    state::notebook::{NotebookState, note},
};

pub async fn consume(
    _db: &mut Db,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NotebookEvent::*;

    match event {
        Key(KeyEvent::Esc) | Notebook(ViewNote) => note::view(state).await,
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Wip("todo: Notebook::consume".to_owned())),
    }
}
