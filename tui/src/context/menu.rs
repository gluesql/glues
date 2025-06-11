use ratatui::widgets::ListState;

pub const CANCEL: &str = "[Esc] Cancel";
pub const QUIT: &str = "[q] Quit";

pub const MENU_ITEMS: [&str; 2] = [CANCEL, QUIT];

pub struct MenuContext {
    pub list_state: ListState,
}

impl Default for MenuContext {
    fn default() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(0)),
        }
    }
}
