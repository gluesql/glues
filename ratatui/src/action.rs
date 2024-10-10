use glues_core::{EntryEvent, Event};

pub enum Action {
    Tui(TuiAction),
    Dispatch(Event),
    PassThrough,
    None,
}

pub enum TuiAction {
    Quit,
}

impl From<TuiAction> for Action {
    fn from(action: TuiAction) -> Self {
        Self::Tui(action)
    }
}

impl From<EntryEvent> for Action {
    fn from(event: EntryEvent) -> Self {
        Self::Dispatch(event.into())
    }
}
