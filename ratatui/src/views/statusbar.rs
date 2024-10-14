use {
    crate::logger::*,
    glues_core::state::State,
    ratatui::{
        layout::{Constraint::Min, Flex, Layout, Rect},
        style::Stylize,
        text::Text,
        Frame,
    },
};

pub fn draw(frame: &mut Frame, area: Rect, state: &State) {
    let shortcuts = state.shortcuts().join(", ");
    let description = state.describe().log_unwrap() + " ";

    let [shortcuts_area, description_area] =
        Layout::horizontal([Min(shortcuts.len() as u16), Min(description.len() as u16)])
            .flex(Flex::SpaceBetween)
            .areas(area);

    frame.render_widget(Text::raw(shortcuts).black().on_gray(), shortcuts_area);
    frame.render_widget(
        Text::raw(description).right_aligned().black().on_gray(),
        description_area,
    );
}
