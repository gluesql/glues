use {
    crate::color::*,
    glues_core::state::State,
    ratatui::{layout::Rect, style::Stylize, text::Line, Frame},
};

pub fn draw(frame: &mut Frame, area: Rect, state: &State) {
    let state_shortcuts = state.shortcuts();
    let num_shortcuts = state_shortcuts.len();
    let mut shortcuts = vec![" ".into()];

    for (i, shortcut) in state_shortcuts.iter().enumerate() {
        shortcuts.push(shortcut.clone().fg(BLACK));

        if i < num_shortcuts - 1 {
            shortcuts.push(" | ".fg(GRAY_LIGHT));
        }
    }

    let shortcuts: Line = shortcuts.into();

    frame.render_widget(shortcuts.bg(GRAY_WHITE), area);
}
