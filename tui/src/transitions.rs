mod entry;
mod keymap;
mod notebook;

use {
    super::{logger::*, App},
    async_recursion::async_recursion,
    glues_core::transition::Transition,
    std::time::SystemTime,
};

impl App {
    #[async_recursion(?Send)]
    pub(super) async fn handle_transition(&mut self, transition: Transition) {
        if self.context.keymap.is_some() {
            self.context.keymap = self
                .glues
                .state
                .keymap
                .then(|| self.glues.state.shortcuts());
        }

        match transition {
            Transition::Keymap(transition) => {
                self.handle_keymap_transition(transition).await;
            }
            Transition::Entry(transition) => {
                self.handle_entry_transition(transition).await;
            }
            Transition::Notebook(transition) => {
                self.handle_notebook_transition(transition).await;
            }
            Transition::Log(message) => {
                log!("{message}");
                self.context.last_log = Some((message, SystemTime::now()));
            }
            Transition::Error(message) => {
                log!("[Err] {message}");
                self.context.alert = Some(message);
            }
        }
    }
}
