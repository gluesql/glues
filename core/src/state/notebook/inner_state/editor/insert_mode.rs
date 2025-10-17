use crate::{
    Error, Event, KeyEvent, NotebookEvent, NotebookTransition, Result,
    backend::CoreBackend,
    state::notebook::{NotebookState, note},
    types::{KeymapGroup, KeymapItem},
};

pub fn consume<B: CoreBackend + ?Sized>(
    _db: &mut B,
    state: &mut NotebookState,
    event: Event,
) -> Result<NotebookTransition> {
    use Event::*;
    use NotebookEvent::*;

    match event {
        Key(KeyEvent::Esc) | Notebook(ViewNote) => note::view(state),
        event @ Key(_) => Ok(NotebookTransition::Inedible(event)),
        _ => Err(Error::Todo(
            "Notebook::EditingInsertMode::consume".to_owned(),
        )),
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
