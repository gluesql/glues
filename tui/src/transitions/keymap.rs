use {crate::App, glues_core::transition::KeymapTransition};

impl App {
    pub(super) async fn handle_keymap_transition(&mut self, transition: KeymapTransition) {
        match transition {
            KeymapTransition::Show => {
                let keymap = self.glues.state.shortcuts();
                self.context.keymap = Some(keymap);
            }
            KeymapTransition::Hide => {
                self.context.keymap = None;
            }
            KeymapTransition::None => {}
        }
    }
}
