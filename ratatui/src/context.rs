mod entry;
pub mod notebook;

pub use {entry::EntryContext, notebook::NotebookContext};

pub struct Context {
    pub entry: EntryContext,
    pub notebook: NotebookContext,
}

impl Context {
    pub fn new() -> Self {
        Self {
            entry: EntryContext::new(),
            notebook: NotebookContext::new(),
        }
    }
}
