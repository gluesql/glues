use {
    crate::logger::*,
    glues_core::state::State,
    ratatui::{
        layout::{
            Constraint::{Min, Percentage},
            Flex, Layout, Rect,
        },
        style::Stylize,
        text::{Line, Text},
        Frame,
    },
};

pub fn draw(frame: &mut Frame, area: Rect, state: &State) {
    let mut shortcuts = vec![" ".into()];

    for (i, shortcut) in state.shortcuts().iter().enumerate() {
        shortcuts.push(shortcut.black());

        if i < state.shortcuts().len() - 1 {
            shortcuts.push(" | ".light_blue().on_gray());
        }
    }

    let shortcuts: Line = shortcuts.into();
    let description = state.describe().log_unwrap() + " ";

    let [shortcuts_area, description_area] =
        Layout::horizontal([Percentage(100), Min(description.len() as u16)])
            .flex(Flex::SpaceBetween)
            .areas(area);

    frame.render_widget(shortcuts.on_gray(), shortcuts_area);
    frame.render_widget(
        Text::raw(description).right_aligned().black().on_gray(),
        description_area,
    );
}
