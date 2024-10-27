use {
    glues_core::state::State,
    ratatui::{layout::Rect, style::Stylize, text::Line, Frame},
};

pub fn draw(frame: &mut Frame, area: Rect, state: &State) {
    let state_shortcuts = state.shortcuts();
    let num_shortcuts = state_shortcuts.len();
    let mut shortcuts = vec![" ".into()];

    for (i, shortcut) in state_shortcuts.iter().enumerate() {
        shortcuts.push(shortcut.clone().black());

        if i < num_shortcuts - 1 {
            shortcuts.push(" | ".light_blue().on_gray());
        }
    }

    let shortcuts: Line = shortcuts.into();

    frame.render_widget(shortcuts.on_gray(), area);
}
