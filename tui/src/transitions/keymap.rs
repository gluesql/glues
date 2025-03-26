use {crate::App, glues_core::transition::KeymapTransition};

impl App {
    pub(super) async fn handle_keymap_transition(&mut self, transition: KeymapTransition) {
        match transition {
            KeymapTransition::Show => {
                self.context.keymap = true;
            }
            KeymapTransition::Hide => {
                self.context.keymap = false;
            }
        }
    }
}
