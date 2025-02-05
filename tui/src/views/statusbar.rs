use {
    crate::{
        color::*,
        context::{notebook::ContextState, NotebookContext},
        logger::*,
    },
    glues_core::state::State,
    ratatui::{
        layout::{
            Constraint::{Length, Percentage},
            Layout, Rect,
        },
        style::Stylize,
        text::{Line, Span, Text},
        Frame,
    },
};

pub fn draw(frame: &mut Frame, area: Rect, state: &State, context: &NotebookContext) {
    let description = format!(" {}", state.describe().log_unwrap());
    let insert_mode = matches!(context.state, ContextState::EditorInsertMode);
    let [desc_area, keymap_area] =
        Layout::horizontal([Percentage(100), Length(if insert_mode { 23 } else { 18 })])
            .areas(area);

    frame.render_widget(
        Text::raw(description).fg(GRAY_DARK).bg(GRAY_WHITE),
        desc_area,
    );

    frame.render_widget(
        Line::from(vec![
            Span::raw("").fg(GREEN).bg(GRAY_WHITE),
            Span::raw(if insert_mode {
                " [Ctrl+h] Show keymap "
            } else {
                " [?] Show keymap "
            })
            .fg(BLACK)
            .bg(GREEN),
        ]),
        keymap_area,
    );
}
