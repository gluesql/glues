use crate::{
    Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
    db::CoreBackend,
    state::notebook::{NotebookState, note},
    types::{KeymapGroup, KeymapItem},
};

pub async fn consume<B: CoreBackend + ?Sized>(
    _db: &mut B,
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

pub fn keymap() -> Vec<KeymapGroup> {
    vec![KeymapGroup::new(
        "General",
        vec![
            KeymapItem::new("Esc", "Save note and enter normal mode"),
            KeymapItem::new("Ctrl+h", "Show editor keymap"),
        ],
    )]
}
