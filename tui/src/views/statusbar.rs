use {
    crate::logger::*,
    glues_core::state::State,
    ratatui::{layout::Rect, style::Stylize, text::Text, Frame},
};

pub fn draw(frame: &mut Frame, area: Rect, state: &State) {
    let description = state.describe().log_unwrap() + " ";

    frame.render_widget(Text::raw(description).centered().black().on_gray(), area);
}
