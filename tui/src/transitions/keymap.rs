use {crate::App, glues_core::transition::KeymapTransition, ratatui::widgets::ScrollbarState};

impl App {
    pub(super) async fn handle_keymap_transition(&mut self, transition: KeymapTransition) {
        match transition {
            KeymapTransition::Show => {
                self.context.keymap = true;
                self.context.keymap_scroll = 0;
                self.context.keymap_scroll_state = ScrollbarState::new(0);
            }
            KeymapTransition::Hide => {
                self.context.keymap = false;
            }
        }
    }
}
